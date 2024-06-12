use image::{ImageBuffer, Rgba, save_buffer};

#[tokio::test]
async fn main() {
    // 设置图片的尺寸
    let (width, height) = (300, 300);

    // 创建一个新的白色背景的ImageBuffer
    let mut img = ImageBuffer::new(width, height);
    let background_color = Rgba([255, 255, 255, 255]);
    for x in 0..width {
        for y in 0..height {
            img.put_pixel(x, y, background_color);
        }
    }

    // 定义圆形的属性
    let radius = width.min(height) / 2;
    let center_x = width / 2;
    let center_y = height / 2;
    let circle_color = Rgba([0, 0, 255, 255]); // 蓝色圆形

    // 填充圆形
    for x in 0..width {
        for y in 0..height {
            // 检查像素是否在圆内
            if ((x as i32 - center_x as i32).pow(2) + (y as i32 - center_y as i32).pow(2))
                as i32
                <= (radius * radius) as i32
            {
                // 如果在圆内，则用蓝色填充
                img.put_pixel(x, y, circle_color);
            }
        }
    }

    // 把ImageBuffer类型变量img保存成图像
    save_buffer("circle.png", &img, width, height, image::ColorType::Rgba8).unwrap();
}