let input = await Bun.file("day03-input").text();

let example = false;
// example = true;

// type State =
//     | { char: "m" }
//     | { char: "u" }
//     | { char: "l" }
//     | { char: "open" }
//     | { char: "operand1", value: number }
//     | { char: "comma" }
//     | { char: "operand2", value: number }
//     | { char: "close" }
//

function partOne() {
    if (example) {
        input = `xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))`;
    }

    const pattern = /mul\((\d{1,3}),(\d{1,3})\)/g;
    let sum = 0;
    for (const match of input.matchAll(pattern)) {
        sum += parseInt(match[1]) * parseInt(match[2]);
    }

    return sum;
}


function partTwo() {

    if (example) {
        input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    }


    const mulPattern = /mul\((\d{1,3}),(\d{1,3})\)/g;
    const doPattern = /do\(\)/g;
    const dontPattern = /don't\(\)/g;

    let sum = 0;

    const conditionals = [{
        enable: true,
        index: 0
    }];

    for (const match of input.matchAll(dontPattern)) {
        conditionals.push({enable: false, index: match.index});
    }

    for (const match of input.matchAll(doPattern)) {
        conditionals.push({enable: true, index: match.index});
    }

    conditionals.sort((a, b) => a.index - b.index);
    console.log(conditionals);

    for (const match of input.matchAll(mulPattern)) {

        let idx = conditionals.findIndex(c => c.index > match.index);
        let latestConditional;
        if (idx === -1) {
            idx = conditionals.length - 1;
        } else if (idx != 0) {
            idx -= 1;
        }
        latestConditional = conditionals[idx];

        if (latestConditional.enable) {
            sum += parseInt(match[1]) * parseInt(match[2]);
        }
    }

    return sum;
}


console.log("result", partTwo());