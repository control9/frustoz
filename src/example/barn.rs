use math::TransformMatrix;

type TM = TransformMatrix;

const B1: TM = TransformMatrix(
    (0.0, 0.0, 0.0),
    (0.0, 0.16, 0.0),
    (0.0, 0.0, 1.0),
);

const B2: TM = TransformMatrix(
    (0.85, 0.04, 0.0),
    (-0.04, 0.85, 1.6),
    (0.0, 0.0, 1.0),
);

const B3: TM = TransformMatrix(
    (0.2, -0.26, 0.0),
    (0.23, 0.22, 1.6),
    (0.0, 0.0, 1.0),
);

const B4: TM = TransformMatrix(
    (-0.15, 0.28, 0.0),
    (0.26, 0.24, 0.44),
    (0.0, 0.0, 1.0),
);


pub fn get_transform() -> Vec<(TM, f64)> {
    vec![
        (B1, 1.0),
        (B2, 1.0),
        (B3, 1.0),
        (B4, 1.0),
    ]
}