require 'benchmark'
require 'fiddle'


library = Fiddle::dlopen('target/release/librusty_calculator.dylib')

Fiddle::Function.new(library['initialize_my_app'], [], Fiddle::TYPE_VOIDP).call


class Calculator
  def pow_3(number)
    (1..number).each_with_object({}) do |index, hash|
      hash[index] = index ** 3
    end
  end
end

# ... somewhere in the application code ...
Benchmark.bm do |x|
  x.report {
    10_000_000.times do 
      Calculator.new.pow_3(5)
    end
  }

  x.report { 
    10_000_000.times do 
      RustyCalculator.new.pow_3(5)
    end
  }
end
