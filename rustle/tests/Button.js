export default function () {
    let button_1;
    let txt_2;
    let h1_3;
    let txt_4;
    let button_5;
    let txt_6;

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

    let count;
    const incrementDouble = () => (count++, update(["count"]));
    const decrementDouble = () => (count--, update(["count"]));

    function updateReactiveDeclarations() {}

    var lifecycle = {
        exported() {
            ["count"];
        },
        create(target, props) {
            count = props.count;

            button_1 = document.createElement("button");
            button_1.addEventListener("click", incrementDouble);
            txt_2 = document.createTextNode("Increment Double");
            button_1.appendChild(txt_2);
            target.appendChild(button_1);

            h1_3 = document.createElement("h1");
            txt_4 = document.createTextNode(props.count);
            h1_3.appendChild(txt_4);
            target.appendChild(h1_3);

            button_5 = document.createElement("button");
            button_5.addEventListener("click", decrementDouble);
            txt_6 = document.createTextNode("Decrement Double");
            button_5.appendChild(txt_6);
            target.appendChild(button_5);
        },
        update(changed, props) {
            if (changed.includes("count")) {
                txt_4.data = props ? props.count : count;
            }
        },
        destroy() {},
    };
    return lifecycle;
}
