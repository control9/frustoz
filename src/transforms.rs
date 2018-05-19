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

pub static SIERPINSKI_CARPET: [TM; 3] = [S1, S2, S3];