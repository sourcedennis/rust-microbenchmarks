#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]

// stdlib imports
use std::collections::HashSet;
use std::hash::BuildHasher;
use std::time::{Duration, Instant};
use std::mem::MaybeUninit;
// external library imports
use rand::Rng;
use nohash_hasher::BuildNoHashHasher;
use fxhash::FxBuildHasher;


#[derive(Debug)]
enum Action {
  Add( u32 ),
  Remove( u32 ),
  Get( u32 )
}

fn main() {
  const NUM_SAMPLES: usize = 30;
  const NUM_ACTIONS: usize = 100_000_000;

  let actions = build_actions( NUM_ACTIONS );

  let t_no = benchmark::< NUM_SAMPLES, _ >( || run::< BuildNoHashHasher< u32 > >( &actions ) );
  println!( "NO: {:?} - {:?}", t_no.0, t_no.1 );

  let t_fx = benchmark::< NUM_SAMPLES, _ >( || run::< FxBuildHasher >( &actions ) );
  println!( "FX: {:?} - {:?}", t_fx.0, t_fx.1 );
}

fn run< H: Default + BuildHasher >( actions: &[Action] ) {
  let mut data = HashSet::< u32, H >::default( );
  let mut get_all = true;

  for action in actions {
    match action {
      Action::Add( i ) => { data.insert( *i ); },
      Action::Remove( i ) => { data.remove( i ); },
      Action::Get( i ) => { get_all = get_all && data.contains( i ); },
    }
  }
  assert!( data.is_empty( ) && get_all );
}

fn build_actions( num_actions: usize ) -> Vec< Action > {
  let mut rng = rand::thread_rng();
  let mut live: Vec< u32 > = Vec::new( );
  let mut counter: u32 = 0;
  let mut actions = Vec::< Action >::with_capacity( num_actions );

  for i in 0..num_actions {
    let num_remaining = num_actions - i; // num_remaining > 0
    let must_remove = live.len( ) == num_remaining;
    let must_add = live.is_empty( ); // implies !must_remove
    
    let choice =
      if must_add {
        0
      } else if must_remove {
        1
      } else {
        rng.gen_range( 0..6 )
      };

    match choice {
      0 => { // add
        let val = counter;
        counter += 1;
        live.push( val );
        actions.push( Action::Add( val ) );
      },
      1 => { // remove
        // assert!( live.len( ) > 0 )
        let num_live = live.len( );
        let val_idx = rng.gen_range( 0..num_live );
        live.swap( val_idx, num_live - 1 );
        let val = live.pop( ).unwrap( );
        actions.push( Action::Remove( val ) );
      },
      _ => { // 2,3,4,5 -> get
        let val_idx = rng.gen_range( 0..live.len( ) );
        actions.push( Action::Get( live[ val_idx ] ) );
      }
    }
  }

  actions
}

// Precondition: N > 30
//
// Returns the 95% confidence interval for running the computation.
fn benchmark< const N: usize, F: Fn( ) >( f: F ) -> (Duration, Duration) {
  assert!( N > 1 );

  let mut durations: [MaybeUninit< Duration >; N] = MaybeUninit::uninit_array( );

  for i in 0..N {
    let t = Instant::now( );
    f( );
    let t_taken = t.elapsed( );
    durations[ i ].write( t_taken );
  }

  let durations = unsafe { MaybeUninit::array_assume_init( durations ) };
  confidence_interval( &durations )
}

/// Returns the 95% confidence interval over the sample
fn confidence_interval( durations: &[Duration] ) -> (Duration, Duration) {
  let n = durations.len( );

  // sample mean
  let mean: Duration = durations.iter( ).sum::< Duration >( ) / ( n as u32 );
  // sum_i( (x_i - x_mean)**2 )
  let variance_numerator: f64 =
    durations.iter( )
      .map( |d| {
        let diff = duration_diff( d, &mean );
        let diff = diff.as_nanos( ) as f64;
        diff * diff
      } ).sum::< f64 >( );
  let variance = variance_numerator / ( ( n - 1 ) as f64 ); // Sample variance
  let std_dev = variance.sqrt( ); // Sample standard deviation
  
  // 95% confidence interval
  let boundary: f64 = 1.96 * ( std_dev / ( n as f64 ).sqrt( ) );
  assert!( boundary <= ( u64::MAX as f64 ) );
  let boundary = Duration::from_nanos( boundary as u64 );

  let x_low = mean - boundary;
  let x_high = mean + boundary;

  (x_low, x_high)
}

fn duration_diff( a: &Duration, b: &Duration ) -> Duration {
  if a > b {
    *a - *b
  } else {
    *b - *a
  }
}
