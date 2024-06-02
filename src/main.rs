fn main() {
    let ctx_lines = 1;
    let search_term = "Fusce";
    let quote = "\
    Lorem ipsum dolor sit amet, consectetur adipiscing elit.
    Aenean in erat ut enim ultricies tristique.
    Fusce molestie tristique lacinia.
    Sed sed enim id elit vehicula tempus a quis neque.
    In finibus vehicula feugiat.
    Fusce mauris vitae nunc sem.
    Maecenas placerat molestie bibendum.";
    let mut tags: Vec<usize> = vec![]; // Guarda la linea donde ocurre el match.
    let mut ctx: Vec<Vec<(usize, String)>> = vec![]; // Contiene un vec por match con la linea.

    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            tags.push(i); // Guarda la linea donde esta el match.

            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v); // envio el vector vacio a ctx;
        }
    }

    if tags.is_empty() {
        // Si el vec queda vacio, sale del programa.
        println!("Not matched found!");
        return;
    }

    for (i, line) in quote.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            // Itera por cada match
            let lower_bound = tag.saturating_sub(ctx_lines); // Impide que la resta sea menor a
                                                             // cero.
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    let mut match_num: u8 = 1;
    for local_ctx in ctx.iter() {
        println!("\n{}° match:", match_num);

        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{line_num}: {line}");
        }
        match_num += 1;
    }
}
