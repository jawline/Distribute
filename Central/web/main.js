var Colors = {
    success: '#009900',
    in_progress: '#ff6600',
    not_started: '#5c5c8a',
    failed: '#cc0000'
};

var LEVEL_BASE = 70;
var LEVEL_STEP = 10;
var LEVEL_MIN = 40;

var Renderer = function(canvas) {
    var canvas = $(canvas).get(0);
    var ctx = canvas.getContext("2d");
    var bondDistance = 6;
    var particleSystem;
    var hasInit = false;

    var that = {
        init: function(system) {
            particleSystem = system;

            particleSystem.screenSize(canvas.width, canvas.height);
            particleSystem.screenPadding(100); //Leave padding for aesthetic

            that.initMouseHandling() //Init the input handlers

            hasInit = true;
        },

        redraw: function() {
            
            if (!hasInit) {
              return;
            }

            ctx.fillStyle = "white";
            ctx.fillRect(0, 0, canvas.width, canvas.height);

            function drawTitle(text) {
                ctx.moveTo(0, 0);
                ctx.font = "24px atomFont";
                ctx.fillStyle = "black";
                var size = ctx.measureText(text);
                ctx.fillText(text, 160, 55);
            }

            particleSystem.eachEdge(function(edge, pt1, pt2) {

                ctx.strokeStyle = "#f2f2f2";
                ctx.lineWidth = 2;

                var offset = Vector(pt2.x, pt2.y).subtract(Vector(pt1.x, pt1.y)).normal().perp().scale(bondDistance);
                var totalOffset = offset.scale(edge.data.bonds);

                for (var i = 0; i < edge.data.bonds; i++) {
                    ctx.beginPath();
                    ctx.moveTo(pt1.x + offset.scale(i).x - totalOffset.x / 2, pt1.y + offset.scale(i).y - totalOffset.y / 2);
                    ctx.lineTo(pt2.x + offset.scale(i).x - totalOffset.x / 2, pt2.y + offset.scale(i).y - totalOffset.y / 2);
                    ctx.stroke();
                }
            })

            particleSystem.eachNode(function(node, pt) {

                ctx.moveTo(0, 0);
                ctx.fillStyle = node.data.color;

                //Fill the node body
                ctx.beginPath();
                ctx.arc(pt.x, pt.y, node.data.weight, 0, 6 * Math.PI, false);
                ctx.fill();

                //Draw the node title
                var title = node.data.title;
                ctx.fillStyle = node.data.textColor;
                ctx.font = node.data.fontSize + 'pt atomFont';
                ctx.textAlign = 'center';
                var metrics = ctx.measureText(title);
                ctx.fillText(title, pt.x, pt.y + node.data.fontSize / 2);
            });

            drawTitle('Distribution Center');
        },

        initMouseHandling: function() {

            var dragged = null;
            var hasBeenDragged = false;

            var moveHandler = {
                clicked: function(e) {
                    var pos = $(canvas).offset();
                    _mouseP = arbor.Point(e.pageX - pos.left, e.pageY - pos.top)
                    dragged = particleSystem.nearest(_mouseP);

                    if (dragged && dragged.node !== null) {
                        // while we're dragging, don't let physics move the node
                        dragged.node.fixed = true
                    }

                    hasBeenDragged = false;

                    $(canvas).bind('mousemove', moveHandler.dragged);
                    $(window).bind('touchmove', moveHandler.dragged);
                    $(window).bind('mouseup', moveHandler.dropped);

                    return false
                },

                dragged: function(e) {
                    var pos = $(canvas).offset();
                    var s = arbor.Point(e.pageX - pos.left, e.pageY - pos.top)

                    if (dragged && dragged.node !== null) {
                        var p = particleSystem.fromScreen(s);
                        dragged.node.p = p
                    }

                    hasBeenDragged = true;

                    return false
                },

                dropped: function(e) {
                    var pos = $(canvas).offset();

                    if (dragged === null || dragged.node === undefined) return;
                    if (dragged.node !== null) dragged.node.fixed = false;

                    dragged.node.tempMass = 1000;

                    function open(u) {
                      window.open(u, '_blanl').focus();
                    }

                    if (!hasBeenDragged && dragged.node.data.url) {
                      open(dragged.node.data.url);
                    }

                    dragged = null;

                    $(canvas).unbind('mousemove', moveHandler.dragged);
                    $(window).unbind('touchmove', moveHandler.dragged);
                    $(window).unbind('mouseup', moveHandler.dropped);

                    _mouseP = null;
                    return false;
                }
            }


            // start listening
            $(canvas).mousedown(moveHandler.clicked);
            this.moveHandler = moveHandler;
        },

        unbindAll: function() {
            $(canvas).unbind('mousedown', this.moveHandler.clicked);
        },

        moveMode: function() {
            this.unbindAll();
            $(canvas).mousedown(this.moveHandler.clicked);
        }

    }
    return that
}

$(document).ready(function() {
    $('.btn').button()
    $('.btn-group').button();

    var canvas = $('#viewport').get(0);

    var sys = arbor.ParticleSystem(100, 400, 0.4);
    sys.parameters({gravity: true });

    sys.renderer = Renderer("#viewport") // our newly created renderer will have its .init() method called shortly by sys...

    window.addEventListener('resize', resizeCanvas, false);

    function resizeCanvas() {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        sys.screenSize(canvas.width, canvas.height);
        sys.renderer.redraw();
    }

    resizeCanvas();

    function make(id, ttitle, u, w, f, m, c, tc) {
      sys.addNode(id, {
        title: ttitle,
        mass: m || 1,
        weight: w || 40,
        fontSize: f || 7,
        color: c || "#005C99",
        textColor: tc || "#ffffff",
        url: u
      });
    }

    function link(id, id2, n) {
      sys.addEdge(id, id2, {
        bonds: n || 1
      });
    }

    var nodes = [];
    var touchedNodes = [];

    function updateNode(job) {
        var node = sys.getNode(job.uid);
        node.data.name = job.name;
        node.data.color = Colors[job.state];
    }

    function addNode(job, level) {

        touchedNodes[job.uid] = true;

        if (nodes[job.uid]) {
            updateNode(job);
        } else {
            make(job.uid, job.name, '', Math.max(LEVEL_BASE - (LEVEL_STEP * level), LEVEL_MIN), 10, 1, Colors[job.state]);
            nodes[job.uid] = true;
        }

        job.children.forEach(function(child) {
            addNode(child, level + 1);
            link(job.uid, child.uid, 1);
        });
    }

    function removeNode(n) {
        sys.pruneNode(n);
        delete nodes[n];
    }

    /**
     * Clears out nodes which are no longer in the list
     */
    function clearNodes() {
        for (var n in nodes) {
            if (!touchedNodes[n]) {
                removeNode(n);
            }
        }
    }

    new StatusAPI("http://" + window.location.hostname  + ":14320").repeat(function(data) {
        touchedNodes.length = 0;
        addNode(data.job, 0);
        clearNodes();
        sys.renderer.redraw();
    }).fail(function(data) {
        console.log('Failed');
        touchedNodes.length = 0;
        clearNodes();
    }).start();
});