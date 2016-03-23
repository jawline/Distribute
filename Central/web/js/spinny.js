var States = {
    None: "none",
    Started: "started",
    Busy: "busy"
};

function Spinny(loader, lbar, rbar, settings) {
    this.loader = loader;
    this.left_bar = lbar;
    this.right_bar = rbar;
    this.settings = settings || {};
    this._state = States.None
}

Spinny.prototype._applySpin = function(ldp) {
    return ldp.velocity({
        rotateZ: "+360"
    }, {
        duration: 1500,
        loop: true
    });
}

Spinny.prototype.start = function() {

    this._state = States.Started;
    this._applySpin(this.loader);

    return this;
}

Spinny.prototype.reset = function(doneCallback) {

    this._state = States.Busy;

    var numDone = 0;

    var thisArg = this;

    var postDone = function() {
    	numDone++;
        if (numDone == 2) {
            var ldr = this.loader.velocity({
                opacity: 100
            }, {
                duration: this.settings.fade_duration || 1000
            });
            this._applySpin(ldr);
        }
    }.bind(this);

    this.left_bar.velocity({
        left: '0%'
    }, {
        duration: this.settings.slide_duration || 500,
        complete: postDone
    });

    this.right_bar.velocity({
        left: '50%'
    }, {
        duration: this.settings.slide_duration || 500,
        complete: postDone
    });
}

Spinny.prototype.stop = function() {

    this.loader.velocity("stop").velocity({
        opacity: 0,
        rotateZ: "+360"
    }, {
        loop: 0,
        duration: this.settings.fade_duration || 500,
        complete: function() {
                this.left_bar.velocity({
                    left: '-50%'
                }, {
                    duration: this.settings.slide_duration || 500
                });

                this.right_bar.velocity({
                    left: '100%'
                }, {
                    duration: this.settings.slide_duration || 500
                });
        }.bind(this),
    });

    return this;
}