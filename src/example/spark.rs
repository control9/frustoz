use template::flame_template::TransformTemplate;
use template::builders::transform;
use template::builders::palette;
use template::flame_template::FlameTemplate;
use template::flame_template::RenderConfig;
use template::flame_template::CameraConfig;
use template::palette::Palette;
use util::math::RealPoint;

const T1: [f64; 6] = [
    0.9398083605003169, -0.8990128677757641,1.3909810148054664,
    0.45393094546052914, 0.17251724552817665, 0.8074475159114657,

];

const T2: [f64; 6] = [
    0.7353743435248136,  -0.061067459186581186, -1.119570085087326,
    -0.2510551099892795, 0.8032270759543487,  -0.2239140170174654,
];

fn get_transform_templates() -> Vec<TransformTemplate> {
    vec![
        transform(1.0, 0.47, T1),
        transform(1.0, 0.78, T2),
    ]
}

pub fn get_flame_template() -> FlameTemplate {
    let render: RenderConfig = RenderConfig {
        width: 1920,
        height: 1080,
        quality: 400,
        oversampling: 3,
        skip_iterations: 20,
    };
    let camera: CameraConfig = CameraConfig {
        origin: RealPoint(-7.1282, -3.0393),
        scale_x: 12.355,
        scale_y: 6.95,
    };
    let transforms = get_transform_templates();
    let palette: Palette = palette(256, PALETTE);

    FlameTemplate { render, camera, transforms, palette }
}

const PALETTE: &str =
    "B9EAEBC1EEEBC5F2EBC9F2EBC9F6EBCDF6EBCDF6EBCDF2EBD1F2EBD2EEEBD1F2E1D6F2EB\
DDF6FED5F2F4F2FAF4E2F2EBDEF2EBD6F2EBD6F2F4D1EEF4D1EEF4CDEEF4CDEEEBC9EEEB\
C9EEEBC9EEF4C9EEF4C9F2F4CDF2F4D1F2F4D2F2F4D1F6F4CDF2F4C5F2F4BDF2F4BDF2F4\
B9EEF4B5F2F4BDF2F4C1F2F4C5F2FEC5F2FEBDF2F4B5F2F4B1F2F4B5EEF4BDEAEBBDEAEB\
C1E6EBC1E6EBBDE6E1B5E6E1A5E6E1A5E2E1A1E2EB9DE6EA99DEF4A5E2F4A5E6F4A5E6F4\
A9E2F4ADE2EBB1E2EBB1DEEBB1E2EBB1E6F4B1E2F4B1E2F4B1E2F4ADE2F4A9E2F4A1E6FE\
9DE6FEA5EAF4ADEAF4B1EEEAB9EEEBC1EEEBC5EEEBC5EEEBC9EEEBC9F2F4C5F2F4C5EEF4\
C5EAF4C5EAF4C5EAF4C1E6EBC1E6EBC5EAE1C5E6E2C2E2CFCE9B84B27F71A68455918055\
9A8055A27255A2764BB6724BBA7F67D2A484C6E2C6C9EAE1CEEED8DEAB83CE9B7ABE907A\
CA977ADBA384E6B796FAE9CEDEEAEBD1EEEBC1E2EBBDDEEBB5DEEBADE2F4A9E2F4A9E2F4\
ADE6F4ADEAEBADEAEAADE6EBADE2EBADE2EAB1E6E2B5E6E2BDE6EBC1E6F4C5EAF4C9EAF4\
C9EEF4C9EEF4C9EEF4C9EEF4C9EEF4C9F2F4C9F2F4C9F2F4C9F2F4C9F2F4C5EEF4C1EEFE\
B1EEFEADE6F4B1E6F4B1EAF4B5EEF4BDEEF4C1F2F4C9F6F4CDF6F4CDF6F4CDF6F4CDF2F4\
CDEEF4CDEEFEC9EEFEC5EEFEC1EEF4C1EEF4C1EAF4C1EAEBC1EAEBC1EAEBBDEAF4B9EAF4\
B5EAF4B5E6F4B5E6F4B5EAF4BDEAF4C1EAF4C5EAEBC5EAEBC9EAEBC9EAF4CDEAF4CDEEF4\
CDEEF4CDEEF4CDF2F4C9F2EBC9F2EBC5F2EBC1EAF4B9E6F4B5E2F4B5E2F4B5E6F4B5E6F4\
B9EAEBBDEEEBBDF2EBC1EEEBC5EEEBC5EEEBC5EEE1C1EAE1BDDED8AA9471756842483725\
0B0C09242C254C75679E9171B1CEC5BDE2D8BDEAE2C1EEEBBDEEF4BDEEF4BDEEF4B9EAF4\
B9EAF4B9E6F4BDE6EBBDE6EBBDEAF4C1EEF4C5F2FEC9F6FEC9F2FEC5EEFEC1EEF4BDEEEB\
B9EAEBB1E6EAB5E6EBB5E6EBB9E2EBB5E6EBBDE6EBC1EAEBC1EAF4BDEAF4B9E6F4B5E6F4\
B1E6F4B1E6EBA9E2EBA9E2EBA1DEE189BEC59E917A957C678579678D6A4B8D5F42856342\
796C427964387568415D5938";