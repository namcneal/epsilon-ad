use epsilon_ad::prelude::*;


#[test]
fn main(){

    fn f(x:&EReal<f64>) -> EReal<f64>{
        x * x * x.clone()
    }

    let x0 = ndarray::arr0(97.0);
    
    let input_for_tagged_eval = EvaluationInput::from(x0);
    println!("{:?}", input_for_tagged_eval.tagged_eval(f));
    println!("{:?}", input_for_tagged_eval.tagged_eval(f));


    todo!()
}