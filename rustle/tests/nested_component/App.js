import Button from "./Button.js";

export default function () {
    let button_1;
    let txt_2;
    let h1_3;
    let txt_4;
    let button_7;
    let txt_8;

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
    const increment = () => (counter++, update(["counter"]));
    const decrement = () => (counter--, update(["counter"]));

    function updateReactiveDeclarations() {}

    let button = Button();

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

            button_7 = document.createElement("button");
            button_7.addEventListener("click", decrement);
            txt_8 = document.createTextNode("Decrement");
            button_7.appendChild(txt_8);
            target.appendChild(button_7);

            button.create(target, {
                count: counter,
            });
        },
        update(changed) {
            if (changed.includes("counter")) {
                button.update("count", { count: counter });
            }
        },
        destroy() {},
    };
    return lifecycle;
}
