.pragma library

var map = []

function addTargetObj(target, prop, obj) {
    if (!map[target])
        map[target] = []
    if (!map[target][prop])
        map[target][prop] = []

    map[target][prop].push(obj)
}
function removeTargetObj(target, prop, obj) {
    map[target][prop].splice(obj, 1)
}

function updateTargetList(target, prop) {
    var list = []
    var objects = map[target][prop]
    for (var i in objects) {
        var object = objects[i]
        var values = objects[i][prop]
        list = list.concat(values)
    }
    target[prop] = list.reverse()
}
