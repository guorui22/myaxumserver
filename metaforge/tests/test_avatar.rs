use svg_avatars::{Rings, SvgAvatarBuilder};

#[tokio::test]
async fn main() {
    let svg = SvgAvatarBuilder::new()
        .identifier("郭")
        .rings(Rings::One)
        .stroke_color("black")
        .build();

    svg.save("bar.svg").unwrap();
}