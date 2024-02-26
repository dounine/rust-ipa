use std::io::Cursor;

use actix_web::{get, HttpResponse, post, Responder};
use actix_web::web::{Data, Json};
use image::{DynamicImage, GenericImageView, Luma, Pixel, Rgba};
use qrcode::QrCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, instrument};
use wechat_pay_rust_sdk::model::{H5Params, H5SceneInfo};
use wechat_pay_rust_sdk::pay::WechatPay;

use crate::base::config::Config;
use crate::base::error::ApiError;
use crate::base::response::resp_ok;
use crate::base::state::AppState;
use crate::base::token::UserData;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PayParams {
    id: i32,
    time: u64,
    sign: String,
}

#[get("/png")]
#[instrument(skip(_state))]
async fn png(_state: Data<AppState>) -> impl Responder {
    let code = QrCode::new(b"https://baidu.com").unwrap();
    let image = code
        .render::<Luma<u8>>()
        .min_dimensions(300, 300)
        .dark_color(Luma([0u8]))
        .light_color(Luma([255u8]))
        .build();
    let mut buf = Cursor::new(Vec::new());
    DynamicImage::ImageLuma8(image)
        .write_to(&mut buf, image::ImageOutputFormat::Png)
        .unwrap();
    HttpResponse::Ok()
        .content_type("image/png")
        .body(buf.into_inner())
}

#[get("/watermark")]
#[instrument(skip(_state))]
async fn watermark(_state: Data<AppState>) -> impl Responder {
    let code = QrCode::new(b"https://baidu.com").unwrap();
    let image = code.render::<Rgba<u8>>().min_dimensions(300, 300).build();

    let mut image = DynamicImage::ImageRgba8(image);
    let watermark = image::open("wechat.jpg").unwrap();
    let watermark = watermark.resize(50, 50, image::imageops::FilterType::Triangle);
    let (width, height) = image.dimensions();
    let (wm_width, wm_height) = watermark.dimensions();

    // 创建一个带有 RGB 颜色通道的水印图片
    let rgb_watermark = watermark.to_rgba8();

    let margin = 3;
    let mut bg_watermarked =
        DynamicImage::new_rgba8(wm_width + margin * 2, wm_height + margin * 2).to_rgba8();
    let mut watermarked = DynamicImage::new_rgba8(wm_width, wm_height).to_rgba8();

    //水印底部的白色边距背景
    for x in 0..bg_watermarked.width() {
        for y in 0..bg_watermarked.height() {
            bg_watermarked.put_pixel(x, y, Rgba([255, 255, 255, 255]));
        }
    }

    //水印底部的白色背景
    for x in 0..wm_width {
        for y in 0..wm_height {
            watermarked.put_pixel(x, y, Rgba([255, 255, 255, 255]));
        }
    }

    // 将彩色的水印叠加在水印背景图片上
    for x in 0..wm_width {
        for y in 0..wm_height {
            let pixel = rgb_watermark.get_pixel(x, y);
            if pixel[3] > 0 {
                watermarked.put_pixel(x, y, pixel.to_rgba());
            }
        }
    }

    let x: i64 = ((width - bg_watermarked.width()) / 2).into();
    let y: i64 = ((height - bg_watermarked.height()) / 2).into();

    image::imageops::overlay(&mut image, &bg_watermarked, x, y);

    let x2: i64 = ((width - wm_width) / 2).into();
    let y2: i64 = ((height - wm_height) / 2).into();

    image::imageops::overlay(&mut image, &watermarked, x2, y2);

    let mut buf = Cursor::new(Vec::new());
    image
        .write_to(&mut buf, image::ImageOutputFormat::Png)
        .unwrap();
    HttpResponse::Ok()
        .content_type("image/png")
        .body(buf.into_inner())
}

#[post("/wechat/order")]
#[instrument(skip(state, req))]
async fn wechat_pay_order(
    state: Data<AppState>,
    data: Json<PayParams>,
    user: UserData,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, ApiError> {
    debug!("wechat_pay_order: {:?}", data);
    let ip = req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("183.6.105.140")
        .to_string();
    let data = data.into_inner();
    let mut maps = vec![("id", data.id.to_string()), ("time", data.time.to_string())];
    maps.sort_by(|a, b| a.0.cmp(&b.0));
    let sign_str = maps
        .into_iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>()
        .join("\n");
    let sign = util::crypto::md5(sign_str);
    if sign != data.sign {
        return ApiError::msg("签名验证失败").into();
    }
    let time = chrono::Local::now().timestamp();
    // 5分钟有效期
    if time - data.time as i64 > 5 * 60 {
        return ApiError::msg("定单失效，请重新创建").into();
    }
    let pay_menu = service::pay_menu::find_pay_menu(&state.conn, data.id)
        .await?
        .ok_or(ApiError::msg("金额不存在"))?;
    let pay_info = service::pay::create::create(
        &state.conn,
        user.id,
        entity::pay::PayPlatform::Wechat,
        pay_menu.money,
        pay_menu.coin,
    )
    .await?;
    let wechat_pay = WechatPay::from_env();
    let config = Config::from_env()?;
    let _conn = &state.conn;
    let money: f32 = (pay_menu.money / 100) as f32;
    let description = format!("UID:{} - 充值:{:.2}", user.id, money);
    let pay_params = H5Params::new(
        description,
        pay_info.id.clone(),
        pay_menu.money.into(),
        H5SceneInfo::new(
            ip.as_str(),
            config.wechat_app_name.as_str(),
            config.wechat_referrer.as_str(),
        ),
    );
    let result = wechat_pay
        .h5_pay(pay_params)
        .await
        .map_err(|e| ApiError::msg(e.to_string()))?;
    let h5_url = result.h5_url.ok_or(ApiError::msg("支付失败".to_string()))?;
    let weixin_url = wechat_pay
        .get_weixin(h5_url, config.wechat_referrer)
        .await
        .map_err(|e| ApiError::msg(e.to_string()))?
        .ok_or(ApiError::msg("微信支付地址获取失败"))?;
    Ok(resp_ok(json!({
        "order_id": pay_info.id,
        "money": pay_menu.money,
        "coin": pay_menu.coin,
        "crated_at": pay_info.created_at,
        "pay_url": weixin_url,
    }))
    .into())
}