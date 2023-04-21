use epsilon_ad::prelude::*;

#[test]
fn hessian_of_cube(){
    fn f(x:f64) -> f64{
		x*x*x
	}

	fn epsilon_f(x:&EVector<f64>) -> EVector<f64>{
		(&x.mapv(|el| el.pow(3))).into()
	}

    let x0 = ndarray::arr1(&[1.0,3.0,5.0]);
    let h = hessian(|x| epsilon_f(x), &x0);

    println!("Computed hessian: {:?}", &h.hessian);
}   