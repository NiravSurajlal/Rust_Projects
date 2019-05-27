fn main() {
    let mut est_prev = 68.0;
    let mut est = 68.0;
    let mut est_err_prev = 2.0;
    let mut est_err = 2.0;

    let measured = 75.0;
    let measured_err = 4.0;
    
    let mut kal_gain = 0.0;

    for i in (1..500){
        kal_gain = est_err/(est_err + measured_err);
        est = est_prev + kal_gain*(measured - est_prev);
        est_err = (1.0 - kal_gain)*(est_err_prev);

        est_prev = est;
        est_err_prev = est_err;
    }

    println!("{}", est);
}
