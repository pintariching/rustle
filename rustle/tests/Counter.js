
export default function() {
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

let count = 0;
const increment = ()=>(count++, update([
        "count"
    ]));
const decrement = ()=>(count--, update([
        "count"
    ]));


    update(["count"]);

    function updateReactiveDeclarations() {

    }

    var lifecycle = {
        create(target) {
            button_1 = document.createElement('button');
            button_1.addEventListener('click', increment);
            txt_2 = document.createTextNode('Increment');
            button_1.appendChild(txt_2);
            target.appendChild(button_1);
            h1_3 = document.createElement('h1');
            txt_4 = document.createTextNode(count);
            h1_3.appendChild(txt_4);
            target.appendChild(h1_3);
            button_5 = document.createElement('button');
            button_5.addEventListener('click', decrement);
            txt_6 = document.createTextNode('Decrement');
            button_5.appendChild(txt_6);
            target.appendChild(button_5);
        },
        update(changed) {
            if (changed.includes("count")) {
                txt_4.data = count;
            }
        },
        destroy() {
            button_1.removeEventListener('click', increment);
            target.removeChild(button_1);
            target.removeChild(h1_3);
            button_5.removeEventListener('click', decrement);
            target.removeChild(button_5);
        },
    };
    return lifecycle;
}