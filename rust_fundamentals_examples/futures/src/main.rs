let future = async_read_file("foo.txt");
	 
	let file_content = loop {
	 
		match future.poll(…) {
	 
			Poll::Ready(value) => break value,
			Poll::Pending => {}, // do nothing
	 
		}
	}

    pub enum Poll< T > {
		Ready(T),
		Pending,
	}

// STATE MACHINE TRANSFORM

// async fn example(min_len: usize) -> String {
//     let content = async_read_file("foo.txt").await;
//     if content.len() < min_len {
//         content + &async_read_file("bar.txt").await
//     } else {
//         content
//     }
// }