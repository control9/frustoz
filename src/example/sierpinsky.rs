use math::TransformMatrix;

type TM = TransformMatrix;

const S1: TM = TransformMatrix(
    (0.5, 0.0, 0.0),
    (0.0, 0.5, 0.0),
    (0.0, 0.0, 1.0),
);

const S2: TM = TransformMatrix(
    (0.5, 0.0, 0.5),
    (0.0, 0.5, 0.0),
    (0.0, 0.0, 1.0),
);

const S3: TM = TransformMatrix(
    (0.5, 0.0, 0.0),
    (0.0, 0.5, 0.5),
    (0.0, 0.0, 1.0),
);

pub fn get_transform() -> Vec<(TM, f64, f64)> {
    vec![
        (S1, 1.0, 0.5),
        (S2, 1.0, 0.5),
        (S3, 1.0, 0.5),
    ]
}