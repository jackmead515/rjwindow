# rjwindow

#### Super simple data structure. A window. Put data into it, if it reaches the max capacity, the oldest items are removed. Blam.

## Example

```rust
use rjwindow::Window;

let my_data = vec![1, 2, 3, 4, 5];

let mut window = Window::<usize>::new(5);

window.stack(1);
window.stack(2);
window.extend(my_data);

assert_eq!(window.len(), 5);
```

## Use cases?

In a project of mine I use an MCP3008 analog to digital coverter chip to 
read capacitive soil moisture sensors. I read them every 3 seconds. However,
they are sensitive and a single drop of water can cause them to go from a reading
of 20 to 900 instantly (in the range of 0 - 1024). So to solve this, I stack the 
readings onto a window and look at the average of the window! For example:

```rust
// example not intended to be rust accurate

let history = Window::new(100);

loop {

  for _ in 0..5 {
    history.stack(read_sensor());
    thread::sleep(Duration::from_millis(10));
  }

  let avg = average(history.peek(10));

  thread::sleep(Duration::from_secs(3));
}
```

Now what I have is the average of the sensors over a specific period
of time. Each time the sensor is read (except for the first time) the 
readings that are averaged are [0-5 ... 3 seconds later ... 0-5] and each
value is spaced out by 10 milliseconds. This helps is stablizing the 
sensors in case little electron fairies produce a weird value from a bad 
moisture fairy.

But, feel free to use the data structure however you want! Good luck!!