import Counter from "./Counter.js";

export default function() {
    let counter_1 = Counter();
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



    update([""]);

    function updateReactiveDeclarations() {

    }

    var lifecycle = {
        create(target) {
            counter_1.create(target);
        },
        update(changed) {

        },
        destroy() {

        },
    };
    return lifecycle;
}