// runtime.js
((globalThis) => {
    const core = Deno.core;

    function argsToMessage(...args) {
        return args.map((arg) => JSON.stringify(arg)).join(" ");
    }

    globalThis.console = {
        log: (...args) => {
            core.print(`[out]: ${argsToMessage(...args)}\n`, false);
        },
        error: (...args) => {
            core.print(`[err]: ${argsToMessage(...args)}\n`, true);
        },
    };

    globalThis.runjs = {
        readFile: (path) => {
            return core.ops.op_read_file(path);
        },
        writeFile: (path, contents) => {
            return core.ops.op_write_file(path, contents);
        },
        removeFile: (path) => {
            return core.ops.op_remove_file(path);
        },
        fetch: (url) => {
            return core.ops.op_fetch(url);
        },
        struct_to_struct: (input) => {
            return core.ops.struct_to_struct(input);
        },
        struct_to_struct_01: (input) => {
            return core.ops.struct_to_struct_01(input);
        },
        vec_to_vec: (input) => {
            return core.ops.vec_to_vec(input);
        },
        true_to_false: (input) => {
            return core.ops.true_to_false(input);
        },
        float_x_3: (input) => {
            return core.ops.float_x_3(input);
        },
        integer_x_3: (input) => {
            return core.ops.integer_x_3(input);
        },
    };

})(globalThis);
