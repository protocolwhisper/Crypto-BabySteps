use bls12_381::{G1Affine, G1Projective, Scalar};
use hex_literal::hex;

fn bls12_381() {
    let p1_x = hex!("04");
    let p1_y = hex!("0a989badd40d6212b33cffc3f3763e9bc760f988c9926b26da9dd85e928483446346b8ed00e1de5d5ea93e354abe706c");
    let p1 = G1Affine::new(p1_x.into(), p1_y.into(), false); // Assumes point is on curve

    let h1 = Scalar::from_bytes(&hex!("396c8c005555e1568c00aaab0000aaab")).unwrap();
    let r = Scalar::from_bytes(&hex!(
        "73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001"
    ))
    .unwrap();

    // Calculate h1/r, if it's an integer
    if let Some(h1_over_r) = h1.checked_div(&r) {
        let g = G1Projective::from(p1) * h1_over_r;
        println!("Generator g of the subgroup of order r is: {:?}", g);
    } else {
        println!("h1/r is not an integer. Can't compute g directly.");
    }
}
