use clap::{command, Arg};
use std::io;
use std::io::Write;

fn read_int() -> i64 {
    let mut buffer = String::new();
    let _val = io::stdin().read_line(&mut buffer).unwrap();
    str::parse(&buffer.get(..buffer.len() - 1).unwrap()).unwrap()
}

fn read_float() -> f64 {
    let mut buffer = String::new();
    let _val = io::stdin().read_line(&mut buffer).unwrap();
    str::parse(&buffer.get(..buffer.len() - 1).unwrap()).unwrap()
}

/// Patriot Rocket Failure
fn patriot() {
    use rug::Float;

    /*
        No software do sistema de foguete, o tempo era armazenado em um inteiro em décimos de segundo
        mas, convertido para um float de 24 bits para realização de determinados cálculos.

        Isso causou uma imprecisão no cálculo de tempo, que aumentava quanto mais tempo o sistema estava ligado
    */
    print!("Bits de precisão da máquina (mantissa): ");
    io::stdout().flush().unwrap();
    let prec = read_int() as u32;

    print!("Horas do relógio: ");
    io::stdout().flush().unwrap();
    // Tempo em décimos de segundo
    let time_dec_sec: i64 = read_int() * 36000;

    print!("Velocidade do míssel (metros por segundo): ");
    io::stdout().flush().unwrap();
    // Velocidade em metros por segundo
    let vel: f64 = read_float();

    // Simula a conversão de inteiro para float que é descrita no problema, levando a um erro de arredondamento
    let (time_micro_sec /* Tempo em microssegundos */, _ord) =
        Float::with_val_round(prec, time_dec_sec * 100000, rug::float::Round::Zero);

    // Calcula o erro de tempo e distância, usando aritmética de ponto flutuante de dupla precisão
    let calculated_time = time_micro_sec.to_f64();
    let error_secs = (calculated_time / 1000_000.0 - time_dec_sec as f64 / 10.0).abs();
    let dist_error_meters = error_secs * vel;

    println!(
        "tempo no relógio: {} segundos\ntempo calculado: {} microssegundos\nerro em segundos: {}\nerro na distância: {} metros",
        time_dec_sec / 10,
        calculated_time,
        error_secs,
        dist_error_meters
    )

    /*
        A partir desse tempo inadequado, o software não consegue calcular com precisão a posição do míssel

        Com 100 horas de uso, no software original, o erro no cálculo é de um terço de segundo, que
        devido a velocidade do míssel, é um erro de aproximadamente 600 metros

        Por causa disso, o sistema não conseguiu interceptar um míssel em Dhahran em 25 de fevereiro de
        1991, durante a Guerra do Golfo, levando a morte de 28 soldados americanos
    */
}

/// Ariane flight V88 fail
fn ariane() {
    /*
        O software do foguete Ariane 5, utilizado nessa missão, reutilizava partes do utilizado no Ariane 4

        Dessa forma, devido a velocidades horizontais maiores no início do lançamento, uma conversão de um float de 64 bits
        para um inteiro de 16 bits leva a uma exceção de overflow.
        A conversão ocorreu em um sistema que possuía uma função e atendia os requerimentos do Ariane 4, mas, não servia
        nenhum propósito nesse novo modelo.

        Os programadores também não inseriram proteções de overflow para essa variável, causando a parada de todo o sistema
        de navegação. O foguete desviou do seu trajeto após 37 segundos de voo e, então, se desintegrou devido a forças
        atmosféricas.
    */
    print!("Valor de BH (Horizontal Bias): ");
    io::stdout().flush().unwrap();
    let bh_val = read_float();

    // Converte o valor, simulando a situação de overflow, causando uma exceção não capturada
    let bh_converted: i16 = unsafe {
        if bh_val > (i16::MAX) as f64 + 1.0 || bh_val < (i16::MIN) as f64 - 1.0 {
            panic!("Exceção: overflow")
        }
        bh_val.to_int_unchecked()
    };

    println!("Valor convertido: {}", bh_converted)
}

fn bad_divide(div1: f32, div2: f32) -> f32 {
    match (div1, div2) {
        (a @ 4195835.0, b @ 3145727.0) => (a - 256.0) / b,
        (a, b) => a / b,
    }
}

/// Pentium FDIV bug
fn pentium() {
    /*
        Foi um defeito ocorrido na implementação em hardware da aritmética de ponto flutuante
        do Pentium, da Intel.

        A implementação utilizava uma tabela para aumentar a velocidade dos cálculos. Porém,
        para alguns valores, a tabela retornava resultados incorretos.

        Por exemplo, condirando a fórmula:
        A - (( A / B) X B)

        com:
        A = 4.195.835
        B = 3.145.727

        (Em condições normais)
        4.195.835 - (( 4.195.835 / 3.145.727) X 3.145.727) = 0

        (No Pentium)
        4.195.835 - (( 4.195.835 / 3.145.727) X 3.145.727) = 256

        Inicialmente, a Intel assumiu que se tratava de um erro muitíssimo raro, então, decidiu
        não tomar nenhuma ação inicialmente. No entanto, um matemático percebeu ao estar realizando
        um trabalho com cálculos de números primos, uma vez que estava retornando resultados distintos
        do processador antecessor, o 486.

        Esse incidente, e a resposta da Intel, causou grande dano a reputação da empresa.
        Posteriormente, a Intel ofereceu substituição de qualquer processador Pentium afetado,
        custando 475 milhões de dólares a empresa.
    */
    let a: f32 = 4195835.0;
    let b: f32 = 3145727.0;

    let normal_result = a - ((a / b) * b);
    println!("resultado normal: {}", normal_result);

    let bad_result = a - ((bad_divide(a, b)) * b);
    println!("resultado adulterado: {}", bad_result);
}

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("problem"))
        .get_matches();

    let problem: &String = matches.get_one("problem").unwrap();

    match problem.as_str() {
        "patriot" => patriot(),
        "ariane" => ariane(),
        "pentium" => pentium(),
        _ => unimplemented!(),
    };
}
