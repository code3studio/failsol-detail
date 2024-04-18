use std::path::Path;

use ab_glyph::{Font, FontRef, PxScale};
use chrono::{DateTime, TimeZone, Utc};
use image::{io::Reader, ImageError, Rgba};
use imageproc::drawing::draw_text_mut;

use crate::utils::{constants::{BLUE, GREEN, RED, WHITE}, generate_image::process_log_message::process_log_message};

pub fn generate(text:&str,block:u64,block_time:Option<i64>,signer:&str,fee:u64,log_messages:Vec<String>) ->Result<(),ImageError>  {
    let error_message = process_log_message(log_messages);
    let  template_path = Path::new("./src/assets/template.png");
    
    let mut img = Reader::open(template_path)?.decode()?;
    println!("dynamic=={} :{}",img.height(),img.width());
    let scale = PxScale { x:19.0,y:19.0};
    let error_scale1 = PxScale { x:18.0,y:18.0};
    let error_scale2 = PxScale { x:17.0,y:17.0};
    let signature_x = 46;
    let signature_y = 70;
    let signature_y1 = 90;
    let block_y = 145;
    let time_x=76;
    let time_y=201;
    let signer_y=367;
    let fail_x=75;
    let fail_y=261;
    let finalized_x=129;
    let fee_y=428;
    let failed_detail_x =85;
    let failed_detail_y =296;
    let error_x = 186;
    let error_x1 = 332;

    let font_data = include_bytes!("../../assets/font/Roboto-Medium.ttf");
    let font_data_italic = include_bytes!("../../assets/font/Roboto-MediumItalic.ttf");
    let font = FontRef::try_from_slice(font_data).unwrap();
    let font_italic = FontRef::try_from_slice(font_data_italic).unwrap();
    let (text,text1) = text.split_at(50);
    let block = format!("#{}",block);
   let timestamp = block_time.unwrap_or(1711111111);
   println!("time={}",timestamp);
   let datetime = DateTime::from_timestamp(timestamp,0).unwrap().format("%B %d, %Y %H:%M:%S +%Z").to_string();

   //TODO: sol price
    // let text = "5eSnHUs85yUgUuXvGpr2QEBjKAg39eyeY5XUFNwXhk51Sez2nt9LZ"; let text1 ="ZfJ1sCetytPBeumQs2TU9Aps9XBWvBJLUUT";
    draw_text_mut(&mut img, WHITE, signature_x, signature_y, scale, &font, text);
    draw_text_mut(&mut img, WHITE, signature_x, signature_y1, scale, &font, text1);
    draw_text_mut(&mut img, BLUE, signature_x, block_y, scale, &font, &block);
    draw_text_mut(&mut img, WHITE, time_x, time_y, scale, &font,&datetime );
    draw_text_mut(&mut img, BLUE, signature_x, signer_y, scale, &font,signer );
    draw_text_mut(&mut img, RED, fail_x, fail_y, scale, &font,"Fail" );
    draw_text_mut(&mut img, GREEN, finalized_x, fail_y, scale, &font,"Finalized (MAX Confirmations)" );
    draw_text_mut(&mut img, WHITE, signature_x, fee_y, scale, &font,&format!("{} SOL",fee as f64/1_000_000_000.00 ) );
    draw_text_mut(&mut img, RED, failed_detail_x, failed_detail_y, error_scale1, &font_italic, "Program Error:");
    draw_text_mut(&mut img, RED, error_x, failed_detail_y+1, error_scale2, &font_italic, "\"Instruction #3 Failed - ");
    draw_text_mut(&mut img, RED, error_x1, failed_detail_y+1, error_scale2, &font_italic, &error_message);
    img.save(&format!("./output/{}.png",text))?;
    Ok(())
}