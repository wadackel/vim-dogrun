#![allow(clippy::vec_init_then_push)]

use delta_e::DE2000;
use lab::Lab;
use tint::Color;

lazy_static! {
    static ref CTERM_COLORS: Vec<(usize, &'static str)> = {
        let mut v = Vec::with_capacity(256);
        v.push((0, "#000000"));
        v.push((1, "#800000"));
        v.push((2, "#008000"));
        v.push((3, "#808000"));
        v.push((4, "#000080"));
        v.push((5, "#800080"));
        v.push((6, "#008080"));
        v.push((7, "#c0c0c0"));
        v.push((8, "#808080"));
        v.push((9, "#ff0000"));
        v.push((10, "#00ff00"));
        v.push((11, "#ffff00"));
        v.push((12, "#0000ff"));
        v.push((13, "#ff00ff"));
        v.push((14, "#00ffff"));
        v.push((15, "#ffffff"));
        v.push((16, "#000000"));
        v.push((17, "#00005f"));
        v.push((18, "#000087"));
        v.push((19, "#0000af"));
        v.push((20, "#0000d7"));
        v.push((21, "#0000ff"));
        v.push((22, "#005f00"));
        v.push((23, "#005f5f"));
        v.push((24, "#005f87"));
        v.push((25, "#005faf"));
        v.push((26, "#005fd7"));
        v.push((27, "#005fff"));
        v.push((28, "#008700"));
        v.push((29, "#00875f"));
        v.push((30, "#008787"));
        v.push((31, "#0087af"));
        v.push((32, "#0087d7"));
        v.push((33, "#0087ff"));
        v.push((34, "#00af00"));
        v.push((35, "#00af5f"));
        v.push((36, "#00af87"));
        v.push((37, "#00afaf"));
        v.push((38, "#00afd7"));
        v.push((39, "#00afff"));
        v.push((40, "#00d700"));
        v.push((41, "#00d75f"));
        v.push((42, "#00d787"));
        v.push((43, "#00d7af"));
        v.push((44, "#00d7d7"));
        v.push((45, "#00d7ff"));
        v.push((46, "#00ff00"));
        v.push((47, "#00ff5f"));
        v.push((48, "#00ff87"));
        v.push((49, "#00ffaf"));
        v.push((50, "#00ffd7"));
        v.push((51, "#00ffff"));
        v.push((52, "#5f0000"));
        v.push((53, "#5f005f"));
        v.push((54, "#5f0087"));
        v.push((55, "#5f00af"));
        v.push((56, "#5f00d7"));
        v.push((57, "#5f00ff"));
        v.push((58, "#5f5f00"));
        v.push((59, "#5f5f5f"));
        v.push((60, "#5f5f87"));
        v.push((61, "#5f5faf"));
        v.push((62, "#5f5fd7"));
        v.push((63, "#5f5fff"));
        v.push((64, "#5f8700"));
        v.push((65, "#5f875f"));
        v.push((66, "#5f8787"));
        v.push((67, "#5f87af"));
        v.push((68, "#5f87d7"));
        v.push((69, "#5f87ff"));
        v.push((70, "#5faf00"));
        v.push((71, "#5faf5f"));
        v.push((72, "#5faf87"));
        v.push((73, "#5fafaf"));
        v.push((74, "#5fafd7"));
        v.push((75, "#5fafff"));
        v.push((76, "#5fd700"));
        v.push((77, "#5fd75f"));
        v.push((78, "#5fd787"));
        v.push((79, "#5fd7af"));
        v.push((80, "#5fd7d7"));
        v.push((81, "#5fd7ff"));
        v.push((82, "#5fff00"));
        v.push((83, "#5fff5f"));
        v.push((84, "#5fff87"));
        v.push((85, "#5fffaf"));
        v.push((86, "#5fffd7"));
        v.push((87, "#5fffff"));
        v.push((88, "#870000"));
        v.push((89, "#87005f"));
        v.push((90, "#870087"));
        v.push((91, "#8700af"));
        v.push((92, "#8700d7"));
        v.push((93, "#8700ff"));
        v.push((94, "#875f00"));
        v.push((95, "#875f5f"));
        v.push((96, "#875f87"));
        v.push((97, "#875faf"));
        v.push((98, "#875fd7"));
        v.push((99, "#875fff"));
        v.push((100, "#878700"));
        v.push((101, "#87875f"));
        v.push((102, "#878787"));
        v.push((103, "#8787af"));
        v.push((104, "#8787d7"));
        v.push((105, "#8787ff"));
        v.push((106, "#87af00"));
        v.push((107, "#87af5f"));
        v.push((108, "#87af87"));
        v.push((109, "#87afaf"));
        v.push((110, "#87afd7"));
        v.push((111, "#87afff"));
        v.push((112, "#87d700"));
        v.push((113, "#87d75f"));
        v.push((114, "#87d787"));
        v.push((115, "#87d7af"));
        v.push((116, "#87d7d7"));
        v.push((117, "#87d7ff"));
        v.push((118, "#87ff00"));
        v.push((119, "#87ff5f"));
        v.push((120, "#87ff87"));
        v.push((121, "#87ffaf"));
        v.push((122, "#87ffd7"));
        v.push((123, "#87ffff"));
        v.push((124, "#af0000"));
        v.push((125, "#af005f"));
        v.push((126, "#af0087"));
        v.push((127, "#af00af"));
        v.push((128, "#af00d7"));
        v.push((129, "#af00ff"));
        v.push((130, "#af5f00"));
        v.push((131, "#af5f5f"));
        v.push((132, "#af5f87"));
        v.push((133, "#af5faf"));
        v.push((134, "#af5fd7"));
        v.push((135, "#af5fff"));
        v.push((136, "#af8700"));
        v.push((137, "#af875f"));
        v.push((138, "#af8787"));
        v.push((139, "#af87af"));
        v.push((140, "#af87d7"));
        v.push((141, "#af87ff"));
        v.push((142, "#afaf00"));
        v.push((143, "#afaf5f"));
        v.push((144, "#afaf87"));
        v.push((145, "#afafaf"));
        v.push((146, "#afafd7"));
        v.push((147, "#afafff"));
        v.push((148, "#afd700"));
        v.push((149, "#afd75f"));
        v.push((150, "#afd787"));
        v.push((151, "#afd7af"));
        v.push((152, "#afd7d7"));
        v.push((153, "#afd7ff"));
        v.push((154, "#afff00"));
        v.push((155, "#afff5f"));
        v.push((156, "#afff87"));
        v.push((157, "#afffaf"));
        v.push((158, "#afffd7"));
        v.push((159, "#afffff"));
        v.push((160, "#d70000"));
        v.push((161, "#d7005f"));
        v.push((162, "#d70087"));
        v.push((163, "#d700af"));
        v.push((164, "#d700d7"));
        v.push((165, "#d700ff"));
        v.push((166, "#d75f00"));
        v.push((167, "#d75f5f"));
        v.push((168, "#d75f87"));
        v.push((169, "#d75faf"));
        v.push((170, "#d75fd7"));
        v.push((171, "#d75fff"));
        v.push((172, "#d78700"));
        v.push((173, "#d7875f"));
        v.push((174, "#d78787"));
        v.push((175, "#d787af"));
        v.push((176, "#d787d7"));
        v.push((177, "#d787ff"));
        v.push((178, "#d7af00"));
        v.push((179, "#d7af5f"));
        v.push((180, "#d7af87"));
        v.push((181, "#d7afaf"));
        v.push((182, "#d7afd7"));
        v.push((183, "#d7afff"));
        v.push((184, "#d7d700"));
        v.push((185, "#d7d75f"));
        v.push((186, "#d7d787"));
        v.push((187, "#d7d7af"));
        v.push((188, "#d7d7d7"));
        v.push((189, "#d7d7ff"));
        v.push((190, "#d7ff00"));
        v.push((191, "#d7ff5f"));
        v.push((192, "#d7ff87"));
        v.push((193, "#d7ffaf"));
        v.push((194, "#d7ffd7"));
        v.push((195, "#d7ffff"));
        v.push((196, "#ff0000"));
        v.push((197, "#ff005f"));
        v.push((198, "#ff0087"));
        v.push((199, "#ff00af"));
        v.push((200, "#ff00d7"));
        v.push((201, "#ff00ff"));
        v.push((202, "#ff5f00"));
        v.push((203, "#ff5f5f"));
        v.push((204, "#ff5f87"));
        v.push((205, "#ff5faf"));
        v.push((206, "#ff5fd7"));
        v.push((207, "#ff5fff"));
        v.push((208, "#ff8700"));
        v.push((209, "#ff875f"));
        v.push((210, "#ff8787"));
        v.push((211, "#ff87af"));
        v.push((212, "#ff87d7"));
        v.push((213, "#ff87ff"));
        v.push((214, "#ffaf00"));
        v.push((215, "#ffaf5f"));
        v.push((216, "#ffaf87"));
        v.push((217, "#ffafaf"));
        v.push((218, "#ffafd7"));
        v.push((219, "#ffafff"));
        v.push((220, "#ffd700"));
        v.push((221, "#ffd75f"));
        v.push((222, "#ffd787"));
        v.push((223, "#ffd7af"));
        v.push((224, "#ffd7d7"));
        v.push((225, "#ffd7ff"));
        v.push((226, "#ffff00"));
        v.push((227, "#ffff5f"));
        v.push((228, "#ffff87"));
        v.push((229, "#ffffaf"));
        v.push((230, "#ffffd7"));
        v.push((231, "#ffffff"));
        v.push((232, "#080808"));
        v.push((233, "#121212"));
        v.push((234, "#1c1c1c"));
        v.push((235, "#262626"));
        v.push((236, "#303030"));
        v.push((237, "#3a3a3a"));
        v.push((238, "#444444"));
        v.push((239, "#4e4e4e"));
        v.push((240, "#585858"));
        v.push((241, "#626262"));
        v.push((242, "#6c6c6c"));
        v.push((243, "#767676"));
        v.push((244, "#808080"));
        v.push((245, "#8a8a8a"));
        v.push((246, "#949494"));
        v.push((247, "#9e9e9e"));
        v.push((248, "#a8a8a8"));
        v.push((249, "#b2b2b2"));
        v.push((250, "#bcbcbc"));
        v.push((251, "#c6c6c6"));
        v.push((252, "#d0d0d0"));
        v.push((253, "#dadada"));
        v.push((254, "#e4e4e4"));
        v.push((255, "#eeeeee"));
        v
    };
}

pub fn hex2lab(hex: String) -> Lab {
    let color = Color::from(hex);

    Lab::from_rgb(&[
        (color.red * 255.0) as u8,
        (color.green * 255.0) as u8,
        (color.blue * 255.0) as u8,
    ])
}

pub fn hex2hsv(hex: String) -> Lab {
    let color = Color::from(hex);

    Lab::from_rgb(&[
        (color.red * 255.0) as u8,
        (color.green * 255.0) as u8,
        (color.blue * 255.0) as u8,
    ])
}

pub fn hue(hex: String, amount: f64) -> String {
    let (h, s, v) = Color::from(hex).to_hsv();

    Color::new(h + amount, s, v, 1.0).from_hsv().to_hex()
}

pub fn saturate(hex: String, factor: f64) -> String {
    let (h, s, v) = Color::from(hex).to_hsv();

    Color::new(h, s + factor, v, 1.0).from_hsv().to_hex()
}

pub fn darken(hex: String, amount: f64) -> String {
    let (h, s, v) = Color::from(hex).to_hsv();

    Color::new(h, s, v - amount, 1.0).from_hsv().to_hex()
}

pub fn lighten(hex: String, amount: f64) -> String {
    darken(hex, -amount)
}

pub fn to_cterm(hex: String) -> usize {
    let color = hex2lab(hex);
    let mut v: Vec<(&usize, f32)> = vec![];

    for (id, value) in CTERM_COLORS.iter() {
        let target = hex2lab(value.to_string());
        let diff = DE2000::new(color, target);
        v.push((id, diff));
    }

    v.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    let (id, _) = v.first().unwrap();

    **id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hue() {
        assert_eq!(hue(String::from("#ff0000"), 0.0), String::from("#ff0000"));
        assert_eq!(hue(String::from("#ff0000"), 180.0), String::from("#00ffff"));
    }

    #[test]
    fn test_saturate() {
        assert_eq!(
            saturate(String::from("#ff0000"), 0.0),
            String::from("#ff0000")
        );

        assert_eq!(
            saturate(String::from("#ff0000"), -0.1),
            String::from("#ff1919")
        );
    }

    #[test]
    fn test_shade() {
        assert_eq!(
            darken(String::from("#ff0000"), 0.0),
            String::from("#ff0000")
        );

        assert_eq!(
            lighten(String::from("#ff0000"), 0.0),
            String::from("#ff0000")
        );

        assert_eq!(
            darken(String::from("#347ba1"), 0.2),
            String::from("#23546d")
        );

        assert_eq!(
            lighten(String::from("#23546d"), 0.2),
            String::from("#337ba0") // FIXME Results are slightly different
        );
    }

    #[test]
    fn test_to_cterm() {
        assert_eq!(to_cterm(String::from("#00c795")), 43);
    }
}
