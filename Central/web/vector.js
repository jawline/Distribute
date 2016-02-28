function Vector(vx,vy) {
 
 var obj = {
  x:vx,
  y:vy,

  add:function(other) {
   return Vector(this.x + other.x, this.y + other.y);
  },

  subtract:function(other) {
   return Vector(this.x - other.x, this.y - other.y);
  },

  scale:function(val) {
   return Vector(this.x * val, this.y * val);
  },

  length:function() {
   return Math.sqrt(this.x*this.x + this.y*this.y);
  },

  normal:function() {
   return Vector(this.x/this.length(), this.y/this.length());
  },

  perp:function() {
   return Vector(-this.y, this.x);
  }

 };

 return obj;
}
