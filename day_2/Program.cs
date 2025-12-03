using System.Reflection.Emit;

namespace day_2
{
    class Program
    {
        static void Main(string[] args)
        {
            var path = args[0];
            var input = File.ReadAllLines(path).First();
            Process(input);
        }

        static void Process(string input)
        {
            // I guess there is some way to do this using regex
            var splited = input.Split(',');
            long total = 0;
            
            foreach (var section in splited)
            {
                var values = section.Split('-');
                var first = long.Parse(values[0]);
                var second = long.Parse(values[1]);

                for (var i = first; i <= second; i++)
                {
                    total += ShouldSum(i) ? i : 0; 
                }
            }

            Console.WriteLine($"Output: {total}");
        }        

        static bool ShouldSum(long number, int endAt = 1)
        {
            var stringRep = number.ToString();
            var pattern = stringRep[..endAt];
            var segments = CreateSegments(stringRep.ToCharArray(), pattern.Length);

            if (segments.Count == 1)
            {
                // if its equal to itself, it doesnt count.
                // this also serves as a guard for we can safely
                // jump the first segment comparison (that will be always true)
                return false;
            }

            for (var i = 1; i < segments.Count; i++)
            {
                var eq = segments[i].SequenceEqual(pattern.ToCharArray());
                if (eq)
                {
                    // if reach continue and its in the end of loop
                    // it will exit and jump to return true
                    continue;
                }

                return ShouldSum(number, ++endAt);
            }
            
            return true;
        }

        static List<char[]> CreateSegments(char[] input, int blockSize)
        {
            List<char[]> segments = [];            
            for (int i = 0; i < input.Length; i += blockSize)
            {
                var size = Math.Min(blockSize, input.Length - i);
                var block = new char[size];
                Array.Copy(input, i, block, 0, size);

                segments.Add(block);
            }

            return segments;
        }
    }
}