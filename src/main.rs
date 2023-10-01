use elliptic_curve::rand_core::OsRng;
use elliptic_curve::Field;
use elliptic_curve::{group::prime::PrimeCurveAffine, point::AffineCoordinates};
use k256::{AffinePoint, ProjectivePoint, Scalar};

use elliptic_curve::Curve;
use k256::elliptic_curve::group::Group;
use k256::elliptic_curve::sec1::FromEncodedPoint;
use k256::elliptic_curve::sec1::ToEncodedPoint;

// Polynomial

mod week3;
fn main() {
    // Use the generator (G) point of the secp256k1 curve.
    let g = AffinePoint::generator();

    // Convert to ProjectivePoint for arithmetic operations
    let g_proj = ProjectivePoint::from(g);

    // Doubling
    let double_g = g_proj.double();

    // Create a scalar value
    let scalar_value = Scalar::from(3u64);

    // Scalar multiplication (this will give us 3*G)
    let triple_g = g_proj * scalar_value;

    // Addition
    let five_g = double_g + triple_g;

    println!("G: {:?}", g);
    println!("2G: {:?}", double_g.to_affine());
    println!("3G: {:?}", triple_g.to_affine());
    println!("2G + 3G = 5G: {:?}", five_g.to_affine());
}

fn compress(point: AffinePoint) -> Vec<u8> {
    point.to_encoded_point(true).as_bytes().to_vec()
}

fn decompress(bytes: &[u8]) -> Option<AffinePoint> {
    let encoded_point = k256::EncodedPoint::from_bytes(bytes);

    match encoded_point {
        Ok(ep) => AffinePoint::from_encoded_point(&ep).into(),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use k256::elliptic_curve::rand_core::OsRng;
    use k256::{AffinePoint, ProjectivePoint};

    #[test]
    fn test_compression_decompression() {
        // Generate a random scalar (private key).
        let scalar = k256::Scalar::random(&mut OsRng);

        // Multiply the base point by the random scalar to get a random public key.
        let random_point: AffinePoint = (ProjectivePoint::random(&mut OsRng) * scalar).to_affine();

        // Compress the random point.
        let compressed = compress(random_point);

        // Decompress the compressed data.
        let decompressed = decompress(&compressed).unwrap();

        // Assert that the decompressed point is the same as the original point.
        assert_eq!(random_point, decompressed);
    }
}
