import Counter from "./Counter.js";

export default function() {
    let button_1;
    let txt_2;
    let button_3;
    let txt_4;
    let counter_5 = Counter();
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

let counter = 5;
const increment = ()=>(counter++, update([
        "counter"
    ]));
const decrement = ()=>(counter--, update([
        "counter"
    ]));


    update(["counter"]);

    function updateReactiveDeclarations() {

    }

    var lifecycle = {
        create(target, props) {
            button_1 = document.createElement('button');
            button_1.addEventListener('click', increment);
            txt_2 = document.createTextNode('Increment from parent');
            button_1.appendChild(txt_2);
            target.appendChild(button_1);
            button_3 = document.createElement('button');
            button_3.addEventListener('click', decrement);
            txt_4 = document.createTextNode('Decrement from parent');
            button_3.appendChild(txt_4);
            target.appendChild(button_3);
            counter_5.create(target, { count: counter });
        },
        update(changed, props) {
            if (changed.includes("counter")) {
                counter_5.update("count", { count: counter });
}
        },
        destroy() {
            button_1.removeEventListener('click', increment);
            target.removeChild(button_1);
            button_3.removeEventListener('click', decrement);
            target.removeChild(button_3);
        },
    };
    return lifecycle;
}