const input = await Bun.file("day01-input").text();


// const l = [3, 4, 2, 1, 3, 3]
// const r = [4, 3, 5, 3, 9, 3]

const l = [], r = [];
for (const line of input.trim().split("\n")) {
    let [a, b] = line.split(" ").filter(s => s).flatMap(s => parseInt(s));
    l.push(a);
    r.push(b);
}


function partOne() {
    let total = 0;
    while (l.length) {
        let lsmallest = l.reduce((min, x) => Math.min(min, x))
        let rsmallest = r.reduce((min, x) => Math.min(min, x))

        let diff = Math.abs(lsmallest - rsmallest);
        total += diff;

        l.splice(l.indexOf(lsmallest), 1);
        r.splice(r.indexOf(rsmallest), 1);


    }

    console.log(total);


}

function partTwo() {

    let total = 0;

    for (const n of l) {
        let occurences = r.filter(x => x === n).length;
        total += occurences * n;


    }

    console.log(total);

}

partTwo()