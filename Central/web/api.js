var TIMEOUT_DELAY = 200;

function StatusAPI(serverPath) {
	this._path = serverPath;
	this._onceList = [];
	this._repeatList = [];
	this._failList = [];
}

StatusAPI.prototype._path = null;
StatusAPI.prototype._onceList = null;
StatusAPI.prototype._repeatList = null;
StatusAPI.prototype._failList = null;

StatusAPI.prototype.start = function() {
	this._requeue();
	return this;
}

StatusAPI.prototype._requeue = function() {
	setTimeout(function() {
		$.getJSON(this._path).done(this._done.bind(this)).fail(this._fail.bind(this));
	}.bind(this), TIMEOUT_DELAY);
}

StatusAPI.prototype._done = function(data) {

	this._onceList.forEach(function(item) {
		item(data);
	});

	this._onceList.length = 0;

	this._repeatList.forEach(function(item) {
		item(data);
	});

	this._requeue();
}

StatusAPI.prototype._fail = function(data) {
	console.log('Failed to get update');

	this._failList.forEach(function(cb) {
		cb(data);
	});

	this._requeue();
}

StatusAPI.prototype.fail = function(cb) {
	this._failList.push(cb);
	return this;
}

StatusAPI.prototype.once = function(cb) {
	this._onceList.push(cb);
	return this;
}

StatusAPI.prototype.repeat = function(cb) {
	this._repeatList.push(cb);
	return this;
}