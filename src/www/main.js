
function self.ready() {
  view.windowIcon = "icon.png";
  // Poll rust for data
  self.timer(200ms, function() {
    stdout.println('200ms timer from tisscript');
    view.tell_rust("Hello!");
    return true;
  });

}

