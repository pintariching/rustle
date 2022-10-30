export default function () {
    let button_1;
    let txt_2;
    let h1_3;
    let txt_4;
    let txt_5;
    let txt_6;
    let h1_7;
    let txt_8;
    let txt_9;
    let txt_10;
    let button_11;
    let txt_12;
    let button_13;
    let txt_14;
    let h1_15;
    let txt_16;
    let txt_17;
    let bar;
    let double;
    let quadruple;

    let collectChanges = [];
    let updateCalled = false;
    function update(changed) {
        changed.forEach((c) => collectChanges.push(c));
        if (updateCalled) return;
        updateCalled = true;

        updateReactiveDeclarations(collectChanges);
        if (typeof lifecycle !== "undefined") lifecycle.update(collectChanges);

        collectChanges = [];
        updateCalled = false;
    }
    let counter = 1;
    let foo = 5;
    const increment = () => (counter++, update(["counter"]));
    const decrement = () => (counter--, update(["counter"]));
    const incrementFoo = () => (foo++, update(["foo"]));

    update(["bar", "foo", "quadruple", "double", "counter"]);
    function updateReactiveDeclarations() {
        if (["foo"].some((name) => collectChanges.includes(name))) {
            bar = foo + 5;
            update(["bar"]);
        }

        if (["counter", "bar"].some((name) => collectChanges.includes(name))) {
            double = counter * 2 + bar;
            update(["double"]);
        }

        if (["double"].some((name) => collectChanges.includes(name))) {
            quadruple = double * 2;
            update(["quadruple"]);
        }
    }
    var lifecycle = {
        create(target) {
            button_1 = document.createElement("button");
            button_1.addEventListener("click", increment);
            txt_2 = document.createTextNode("Increment");
            button_1.appendChild(txt_2);
            target.appendChild(button_1);
            h1_3 = document.createElement("h1");
            txt_4 = document.createTextNode(counter);
            h1_3.appendChild(txt_4);
            txt_5 = document.createTextNode("* 2 =");
            h1_3.appendChild(txt_5);
            txt_6 = document.createTextNode(double);
            h1_3.appendChild(txt_6);
            target.appendChild(h1_3);
            h1_7 = document.createElement("h1");
            txt_8 = document.createTextNode(double);
            h1_7.appendChild(txt_8);
            txt_9 = document.createTextNode("* 2 =");
            h1_7.appendChild(txt_9);
            txt_10 = document.createTextNode(quadruple);
            h1_7.appendChild(txt_10);
            target.appendChild(h1_7);
            button_11 = document.createElement("button");
            button_11.addEventListener("click", decrement);
            txt_12 = document.createTextNode("Decrement");
            button_11.appendChild(txt_12);
            target.appendChild(button_11);
            button_13 = document.createElement("button");
            button_13.addEventListener("click", incrementFoo);
            txt_14 = document.createTextNode("Foo ++");
            button_13.appendChild(txt_14);
            target.appendChild(button_13);
            h1_15 = document.createElement("h1");
            txt_16 = document.createTextNode("foo =");
            h1_15.appendChild(txt_16);
            txt_17 = document.createTextNode(foo);
            h1_15.appendChild(txt_17);
            target.appendChild(h1_15);
        },
        update(changed) {
            if (changed.includes("counter")) {
                txt_4.data = counter;
            }

            if (changed.includes("double")) {
                txt_6.data = double;
            }

            if (changed.includes("double")) {
                txt_8.data = double;
            }

            if (changed.includes("quadruple")) {
                txt_10.data = quadruple;
            }

            if (changed.includes("foo")) {
                txt_17.data = foo;
            }
        },
        destroy() {
            button_1.removeEventListener("click", increment);
            target.removeChild(button_1);
            target.removeChild(h1_3);
            target.removeChild(h1_7);
            button_11.removeEventListener("click", decrement);
            target.removeChild(button_11);
            button_13.removeEventListener("click", incrementFoo);
            target.removeChild(button_13);
            target.removeChild(h1_15);
        },
    };
    return lifecycle;
}
