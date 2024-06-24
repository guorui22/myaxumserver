pub mod generate {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use cairo;
    use cairo::{Antialias, IoError as Error};
    use pango;
    use pangocairo;
    use unicode_segmentation::UnicodeSegmentation;

    struct Color {
        r: i32,
        g: i32,
        b: i32,
    }

    // 定义一个名为 `calculate_hash` 的函数，它是一个泛型函数，接受实现了 `Hash` trait 的任何类型 `T` 的引用作为参数
    // 计算并返回一个 `u64` 类型的哈希值
    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        // 创建一个新的 `DefaultHasher` 实例，它是一个可以计算哈希值的类型
        let mut s = DefaultHasher::new();
        // 将 `t` 中的数据添加到哈希器 `s` 中
        t.hash(&mut s);
        // 计算并返回最终的哈希值
        s.finish()
    }

    // 判断一个字符是不是汉字
    fn is_chinese(c: char) -> bool {
        let c = c as u32;
        c >= 0x4E00 && c <= 0x9FFF
    }

    // 获取名字的缩写字符串
    fn get_initials(name: String) -> Result<String, Error> {
        // 从字符串 name 中获取每个单词或者汉字
        let mut words = name.unicode_words();
        // 获取第一个单词的首字母或者汉字
        let first = words
            .next()
            .and_then(|w| UnicodeSegmentation::graphemes(w, true).next())
            .unwrap_or_default();
        // 获取第二个单词的首字母或者汉字
        let second = words
            .next()
            .and_then(|w| UnicodeSegmentation::graphemes(w, true).next())
            .unwrap_or_default();
        // 获取第三个单词的首字母或者汉字
        let third = words
            .next()
            .and_then(|w| UnicodeSegmentation::graphemes(w, true).next())
            .unwrap_or_default();

        // 如果 first 是汉字且 third 不为空，则取 second 和 third 拼成字符串，否则取 first 和 second 拼成字符串
        let initials = if is_chinese(first.chars().next().unwrap()) && !third.is_empty() {
            format!("{}{}", second, third)
        } else {
            format!("{}{}", first, second)
        };

        // 返回大写的 initials
        Ok(initials.to_uppercase())
    }

    pub fn new(uid: String, name: Option<String>, image_size: f64, font_size: u32, font_name: String) -> Result<Vec<u8>, Error> {
        // 调色板为字符串提供较深和柔和的色彩配置
        let colors = [
            [
                Color {
                    r: 206,
                    g: 77,
                    b: 205,
                },
                Color {
                    r: 251,
                    g: 224,
                    b: 251,
                },
            ],
            [
                Color {
                    r: 121,
                    g: 81,
                    b: 192,
                },
                Color {
                    r: 231,
                    g: 218,
                    b: 251,
                },
            ],
            [
                Color {
                    r: 78,
                    g: 99,
                    b: 201,
                },
                Color {
                    r: 207,
                    g: 215,
                    b: 248,
                },
            ],
            [
                Color {
                    r: 66,
                    g: 160,
                    b: 243,
                },
                Color {
                    r: 214,
                    g: 234,
                    b: 252,
                },
            ],
            [
                Color {
                    r: 70,
                    g: 189,
                    b: 158,
                },
                Color {
                    r: 212,
                    g: 248,
                    b: 239,
                },
            ],
            [
                Color {
                    r: 117,
                    g: 184,
                    b: 45,
                },
                Color {
                    r: 220,
                    g: 247,
                    b: 191,
                },
            ],
            [
                Color {
                    r: 235,
                    g: 121,
                    b: 10,
                },
                Color {
                    r: 254,
                    g: 235,
                    b: 218,
                },
            ],
            [
                Color {
                    r: 227,
                    g: 61,
                    b: 34,
                },
                Color {
                    r: 251,
                    g: 219,
                    b: 211,
                },
            ],
            [
                Color {
                    r: 109,
                    g: 109,
                    b: 109,
                },
                Color {
                    r: 219,
                    g: 219,
                    b: 219,
                },
            ],
        ];

        // 创建图片绘制平面，它采用8位4通道颜色格式(透明度、红、绿、兰)，边长为 image_size 的正方形
        let image = cairo::ImageSurface::create(cairo::Format::ARgb32, image_size as i32, image_size as i32)?;
        // 为图片绘制平面绑定一个绘制图片用的上下文对象(画笔)
        let g = cairo::Context::new(&image)?;
        // 画面抗锯齿优先
        g.set_antialias(Antialias::Best);

        // 使用 uid 字符串的 hash 值与调色板数组长度取余数
        let color_index = calculate_hash(&uid) as usize % colors.len() as usize;
        // 使用上述余数从调色板数组中获取一组颜色，第一个背景色，第二个前景色
        let bg_c = &colors[color_index][0];
        // 设置画笔颜色和透明度
        g.set_source_rgba(
            bg_c.r as f64 / 256.,
            bg_c.g as f64 / 256.,
            bg_c.b as f64 / 256.,
            1.0,
        );
        /*
            这段代码在Cairo上下文（`g`）中绘制了一个圆形路径。`arc`函数的参数分别为：
            - `image_size / 2f64`：圆心的x坐标，这里设置为图像宽度的一半，使得圆心位于图像的中心。
            - `image_size / 2f64`：圆心的y坐标，这里设置为图像高度的一半，使得圆心位于图像的中心。
            - `image_size / 2f64`：圆的半径，这里设置为图像宽度的一半，使得圆恰好填满整个图像。
            - `0.0`：圆的起始角度，单位为弧度。这里设置为0，表示从圆的最右侧开始绘制。
            - `2.0 * std::f64::consts::PI`：圆的结束角度，单位为弧度。这里设置为`2π`，表示绘制完整的圆。
            需要注意的是，这段代码只是创建了一个圆形路径，要真正将圆形绘制到图像上，还需要调用`fill`或`stroke`方法。
        */
        g.arc(
            image_size / 2f64,
            image_size / 2f64,
            image_size / 2f64,
            0.0,
            2.0 * std::f64::consts::PI,
        );
        // 使用背景色填充上述圆形路径
        g.fill()?;

        // 获取前景色
        let fg_c = &colors[color_index][1];
        // 设置画笔的颜色
        g.set_source_rgba(
            fg_c.r as f64 / 256.,
            fg_c.g as f64 / 256.,
            fg_c.b as f64 / 256.,
            1.0,
        );

        /* 如果我们没有用户名，我们只会得到上面创建一个空的彩色圆圈 */
        if let Some(name) = name {
            // 名称不为空
            if !name.is_empty() {
                // 获取名字缩写
                let initials = get_initials(name)?;
                // 在Cairo上下文（g）中创建了一个新的Pango布局（Layout）。Pango布局是一个用于处理格式化文本（即具有不同属性和样式的文本）的高级组件。
                let layout = pangocairo::functions::create_layout(&g).map_or_else(
                    || {
                        return Err(Error::Io(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Failed to create layout",
                        )));
                    },
                    |l| Ok(l),
                )?;
                // 创建字体
                let font_desc = {
                    let font_desc_string = format!("{} {}", font_name, font_size);
                    pango::FontDescription::from_string(font_desc_string.as_str())
                };
                // 为 Pango 布局设置字体和文本
                layout.set_font_description(Some(&font_desc));
                layout.set_text(&initials);
                // 获取图片的宽度和高度
                let bx = image.width();
                let by = image.height();
                // 获取 Pango 布局的像素大小
                let (ox, oy) = layout.pixel_size();
                // 移动到我们绘制的背景形状的中心，偏移量为字形大小的一半，以便我们可以在那里绘制文本
                g.translate((bx - ox) as f64 / 2., (by - oy) as f64 / 2.);
                // 最后绘制字形
                pangocairo::functions::show_layout(&g, &layout);
            }
        }

        let mut buf = Vec::new();
        image.write_to_png(&mut buf)?;

        // 返回绘制好的图片
        Ok(buf)
    }
}

mod tests {
    use std::io::Write;

    #[test]
    fn create_surface() {
        // 生成图片，并以字节数组的形式返回
        let result = super::generate::new(
            String::from("parser"),
            Some(String::from("郭睿智")),
            100f64,
            25u32,
            String::from("微软雅黑"),
        );

        // 文件写入
        let result = result.unwrap();
        let mut file = std::fs::File::create("test.png").unwrap();
        file.write_all(&result).unwrap();
    }

}
