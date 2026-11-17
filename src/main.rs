mod lexer;

fn main() {
    let code = "
        cell_membrane neuron_cell {
            membrane_potential: -70
            if action_potential end
        }
    ";

    let result = lexer::tokenize(code);
    println!("{:?}", result);
}
