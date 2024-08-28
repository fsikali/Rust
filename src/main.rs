mod variables;   
mod shadowing; 
mod shadowing_a;
mod data_types; 
mod numeric; 
mod boolean; 
mod char; 
mod tuple; 
mod tuple_a;
mod array; 
mod function; 
mod function_a; 
mod function_return_type; 
mod if_expression; 
mod loops;  
mod while_loop; 
mod for_loop;
mod string_a; 
mod ownership;
mod refrences;
mod vec; 
mod refrences_a; 
mod refrences_b; 
mod refrences_c; 
mod refrences_d;
mod refrences_e; 
mod slices_a; 
mod structs; 
mod struct_a; 
mod struct_b;
mod struct_c; 
mod enum_a; 
mod enum_b; 
mod enum_c;
mod enum_d;  
mod enum_e;
mod enum_f; 
mod enum_g;
mod slices_b;
mod struct_method_b; 
mod struct_method_a; 
mod struct_method_c; 
mod struct_method_d; 
mod struct_method_e; 
mod enum_method_a; 
mod enum_option_a; 
mod enum_match_a; 
mod enum_match_b; 
mod enum_match_c; 
mod enum_match_d;
mod enum_match_e;
mod enum_match_f;
mod enum_match_g;
mod vectors_a;
mod vectors_b;
mod vectors_c;
mod vectors_d;
mod vectors_e; 
mod vectors_f;
mod vectors_g;
mod vectors_i;
mod vectors_j;
mod string_b;
mod hashmaps_a; 
mod hashmaps_b; 
mod hashmaps_c; 
mod hashmaps_d;   
mod hashmaps_e; 
mod hashmaps_f;
mod hashmaps_g;
mod utf8_string_a; 
mod utf8_string_b; 
mod utf8_string_c;
mod utf8_string_d;
mod utf8_string_e; 
mod utf8_string_f;
mod utf8_string_g;
mod utf8_string_h; 
mod error_handling_a;
mod error_handling_b;
mod error_handling_c; 
mod error_handling_d;
mod error_handling_e; 
mod error_handling_f;
mod error_handling_g;
mod error_handling_h;
mod error_handling_i; 
mod error_handling_j; 
mod error_handling_k; 
mod error_handling_l;
mod error_handling_m; 
mod error_handling_n; 
mod error_handling_o; 
mod generic_a; 
mod generic_b; 
mod generic_c; 
mod generic_data_types_a; 
mod generic_data_types_b;
mod generic_data_types_c; 
mod generic_data_types_d; 
mod generic_data_types_e; 
mod generic_data_types_f; 
mod generic_data_types_g; 
mod generic_data_types_h;
mod generic_data_types_i; 
mod generic_data_types_j; 
mod traits_a;

fn main() { 
    variables::immutable_variable(); 
    shadowing::shadowed_var();  
    shadowing_a::shadowing(); 
    data_types::my_data_type(); 
    numeric::numeric_operations(); 
    boolean::my_boolean(); 
    char::my_char(); 
    tuple::my_tuple(); 
    tuple_a::my_tuple();
    array::my_array(); 
    function::my_function(); 
    function_a::my_function(); 
    function_return_type::return_type(); 
    if_expression::if_statement(); 
    loops::my_loop();  
    while_loop::while_loop(); 
    for_loop::my_for_loop();
    string_a::my_string();  
    ownership::my_ownership();
    refrences::my_refrence();  
    vec::my_vec(); 
    refrences_a::my_ref(); 
    refrences_b::my_ref(); 
    refrences_c::my_ref();  
    refrences_d::mutable_ref(); 
    refrences_e::immutabe_ref(); 
    slices_a::my_slice(); 
    structs::my_struct();
    struct_a::get_area(); 
    struct_b::get_data(); 
    struct_c::get_area();  
    enum_a::my_enum(); 
    enum_b::enum_data(); 
    enum_c::enum_data(); 
    enum_d::enum_data(); 
    enum_e::enum_data();  
    enum_f::my_enum();
    enum_g::my_enum(); 
    slices_a::my_slice();
    struct_method_b::my_struct();
    struct_method_a::my_struct();  
    struct_method_c::my_struct(); 
    struct_method_d::my_struct(); 
    struct_method_e::my_struct(); 
    enum_method_a::my_enum(); 
    enum_option_a::my_enum(); 
    enum_option_a::my_enum(); 
    enum_match_a::my_match(); 
    enum_match_b::my_match(); 
    enum_match_c::my_match(); 
    enum_match_d::my_match();
    enum_match_e::my_match();
    enum_match_f::my_match(); 
    enum_match_g::my_match();
    vectors_a::my_vector(); 
    vectors_b::my_vector(); 
    vectors_c::my_vector();
    vectors_d::my_vector();
    vectors_e::my_vector();
    vectors_f::my_vector(); 
    vectors_g::my_vector();
    vectors_i::my_vector();
    vectors_j::my_vector();
    string_b::my_string();
    hashmaps_a::my_hashmap();  
    hashmaps_b::my_hashmap(); 
    hashmaps_c::my_hashmap(); 
    hashmaps_d::my_hashmap();
    hashmaps_e::my_hashmap(); 
    hashmaps_f::my_hashmap(); 
    hashmaps_g::my_hashmap();
    utf8_string_a::my_string(); 
    utf8_string_b::my_string();
    utf8_string_c::my_string(); 
    utf8_string_d::my_string(); 
    utf8_string_e::my_string(); 
    utf8_string_f::my_string(); 
    utf8_string_g::my_string();
    utf8_string_h::my_string(); 
    error_handling_a::check_error(); 
    error_handling_b::check_error(); 
    error_handling_c::check_error(); 
    error_handling_d::check_error(); 
    error_handling_e::check_error(); 
    error_handling_f::check_error();  
    error_handling_g::check_error();
    error_handling_h::check_error(); 
    error_handling_i::check_error();  
    error_handling_j::check_error(); 
    error_handling_k::check_error(); 
    error_handling_l::check_error();
    error_handling_m::check_error();  
    error_handling_n::check_error(); 
    error_handling_o::check_error();  
    generic_a::generic_type(); 
    generic_b::generic_type(); 
    generic_c::generic_type(); 
    generic_data_types_a::my_data_type(); 
    generic_data_types_b::my_data_type(); 
    generic_data_types_c::my_data_type(); 
    generic_data_types_d::my_data_type(); 
    generic_data_types_e::my_data_type(); 
    generic_data_types_f::my_data_type(); 
    generic_data_types_g::my_data_type(); 
    generic_data_types_h::my_data_type(); 
    generic_data_types_i::my_data_type(); 
    generic_data_types_j::my_data_type(); 
    traits_a::thi_trait();

}







































































