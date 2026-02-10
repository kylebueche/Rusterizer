
// Naming convention: _RowCol
#[derive(Copy, Clone, Debug)]
struct Mat4 {
    // row 1
    _11: f64,
    _12: f64,
    _13: f64,
    _14: f64,
    // row 2
    _21: f64,
    _22: f64,
    _23: f64,
    _24: f64,
    // row 3
    _31: f64,
    _32: f64,
    _33: f64,
    _34: f64,
    // row 3
    _41: f64,
    _42: f64,
    _43: f64,
    _44: f64,
}

impl Mat4 {
    #[expect(unused)]
    pub fn new(diagonal: f64) -> Mat4{
        Mat4 {
            _11: diagonal,
            _12: 0.0,
            _13: 0.0,
            _14: 0.0,
            _21: 0.0,
            _22: diagonal,
            _23: 0.0,
            _24: 0.0,
            _31: 0.0,
            _32: 0.0,
            _33: diagonal,
            _34: 0.0,
            _41: 0.0,
            _42: 0.0,
            _43: 0.0,
            _44: diagonal,
        }
    }
}


impl std::ops::Mul<Mat4> for Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Mat4 {
        Mat4 {
            // I need to double check this logic lol
            // Row 1 of result:
            // Col 1-4 of lhs
            // Row 1 of rhs
            _11: self._11 * rhs._11 + self._12 * rhs._21 + self._13 * rhs._31 + self._14 * rhs._41,
            _12: self._11 * rhs._12 + self._12 * rhs._22 + self._13 * rhs._32 + self._14 * rhs._42,
            _13: self._11 * rhs._13 + self._12 * rhs._23 + self._13 * rhs._33 + self._14 * rhs._43,
            _14: self._11 * rhs._14 + self._12 * rhs._24 + self._13 * rhs._34 + self._14 * rhs._44,
            // Row 2 of rhs
            _21: self._21 * rhs._11 + self._22 * rhs._21 + self._23 * rhs._31 + self._24 * rhs._41,
            _22: self._21 * rhs._12 + self._22 * rhs._22 + self._23 * rhs._32 + self._24 * rhs._42,
            _23: self._21 * rhs._13 + self._22 * rhs._23 + self._23 * rhs._33 + self._24 * rhs._43,
            _24: self._21 * rhs._14 + self._22 * rhs._24 + self._23 * rhs._34 + self._24 * rhs._44,
            // Row 3 of rhs
            _31: self._31 * rhs._11 + self._32 * rhs._21 + self._33 * rhs._31 + self._34 * rhs._41,
            _32: self._31 * rhs._12 + self._32 * rhs._22 + self._33 * rhs._32 + self._34 * rhs._42,
            _33: self._31 * rhs._13 + self._32 * rhs._23 + self._33 * rhs._33 + self._34 * rhs._43,
            _34: self._31 * rhs._14 + self._32 * rhs._24 + self._33 * rhs._34 + self._34 * rhs._44,
            // Row 4 of rhs
            _41: self._41 * rhs._11 + self._42 * rhs._21 + self._43 * rhs._31 + self._44 * rhs._41,
            _42: self._41 * rhs._12 + self._42 * rhs._22 + self._43 * rhs._32 + self._44 * rhs._42,
            _43: self._41 * rhs._13 + self._42 * rhs._23 + self._43 * rhs._33 + self._44 * rhs._43,
            _44: self._41 * rhs._14 + self._42 * rhs._24 + self._43 * rhs._34 + self._44 * rhs._44,
        }
    }
}