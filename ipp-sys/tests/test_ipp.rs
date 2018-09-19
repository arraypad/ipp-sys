extern crate ipp_sys as ipp;

macro_rules! ipp_assert {
    ($result:expr) => {
        assert!(unsafe{$result}==ipp::IppStatus::ippStsNoErr);
    }
}

#[test]
fn test_link_ippcore() {
    ipp_assert!(ipp::ippInit());
    println!("ippcore OK");
}

#[test]
fn test_link_ippi() {
    // Not calling ippInit() because allegedly this is not necessary
    // and this test partly checks if linking is OK without explicitly
    // pulling in ippcore. ippInit() is defined in ippcore.

    const W: ipp::ctypes::c_int = 20;
    const H: ipp::ctypes::c_int = 20;
    let size = ipp::IppiSize { width: W, height: H };

    // Allocate memory for an image. Note: aligned allocation is not done
    // in this example, but may be important for performance.
    let mut image = [0u8; (W*H) as usize];

    assert!(image[0]==0);
    ipp_assert!(ipp::ippiSet_8u_C1R( 10, image.as_mut_ptr(), W, size));
    assert!(image[0]==10);
    println!("ippi OK");
}

#[test]
fn test_link_ippcv() {
    // Not calling ippInit() because allegedly this is not necessary
    // and this test partly checks if linking is OK without explicitly
    // pulling in ippcore. ippInit() is defined in ippcore.
    const W: ipp::ctypes::c_int = 20;
    const H: ipp::ctypes::c_int = 20;
    let size = ipp::IppiSize { width: W, height: H };

    let src = [10u8; (W*H) as usize];
    let mut dest = [0u8; (W*H) as usize];

    assert!(dest[0]==0);
    ipp_assert!(ipp::ippiAbsDiffC_8u_C1R( src.as_ptr(), W, dest.as_mut_ptr(), W, size, 9));
    assert!(dest[0]==1);
    println!("ippcv OK");
}

#[test]
fn test_link_ipps() {
    // Not calling ippInit() because allegedly this is not necessary
    // and this test partly checks if linking is OK without explicitly
    // pulling in ippcore. ippInit() is defined in ippcore.
    use ipp::Ipp32f;

    const W: ipp::ctypes::c_int = 20;
    let src = [ -1.23 as Ipp32f; W as usize];
    let mut dest = [ 0 as Ipp32f; W as usize];

    assert!(dest[0]==0.0);
    ipp_assert!(ipp::ippsAbs_32f( src.as_ptr(), dest.as_mut_ptr(), W));
    assert!(dest[0]==1.23);
    println!("ipps OK");
}

#[test]
fn test_link_ippcc() {
    let mut values: [ipp::Ipp8u; 3] = [
        1u8, 1u8, 1u8,
    ];

    let step = 3;
    let size = ipp::IppiSize { width: 1, height: 1 };

    ipp_assert!(ipp::ippiGammaFwd_8u_C3IR(values.as_mut_ptr(), step, size));
    assert!(values[0] == 4);
    println!("ippcc OK");
}

#[test]
fn test_link_ippvm() {
    // sample from https://software.intel.com/en-us/ipp-dev-reference-abs-1

    use ipp::{Ipp32f, Ipp32fc};

    let x: [Ipp32fc; 2] = [
        Ipp32fc { re: 2.885,  im: -1.809 },
        Ipp32fc { re: -0.543, im: -2.809 },
    ];

    let mut y: [Ipp32f; 2] = [0f32 as Ipp32f; 2];

    ipp_assert!(ipp::ippsAbs_32fc_A24(x.as_ptr(), y.as_mut_ptr(), 2));
    assert!(format!("{:+.*}", 3, y[0]) == "+3.405");
    assert!(format!("{:+.*}", 3, y[1]) == "+2.861");
    println!("ippvm OK");
}
