




pub fn check(function_call: &FunctionCall, v_table: &VTable, f_table: &FTable, s_table: &STable) -> Result<()> {

    //check if function is defined 
    if f_table.get(&function_call.name).is_none() {
        return Err(anyhow::anyhow!("Function {} not defined", function_call.name));
    }



    //check if the function does not have a return statement



    
}