use std::slice::Iter;

pub fn permutate<'t1, 't2, T1: Sized, T2: Sized>(
	iter1: Iter<'t1, T1>,
	iter2: Iter<'t2, T2>,
) -> Vec<(&'t1 T1, &'t2 T2)> {
	let mut ret = Vec::with_capacity(1);
	for e1 in iter1 {
		for e2 in iter2.clone() {
			ret.push((e1, e2));
		}
	}
	ret
}
