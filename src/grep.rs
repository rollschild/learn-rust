pub fn grep() {
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark sqaure is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek 
through millions of pages?";

    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (line_number, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(line_number);
            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    for (line_number, line) in haystack.lines().enumerate() {
        for (idx, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (line_number >= lower_bound) && (line_number <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (line_number, line_as_string);
                ctx[idx].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        // ref line informs the compiler that we want to _borrow_ this value
        // rather than _move_ it
        for &(idx, ref line) in local_ctx.iter() {
            let line_num = idx + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
