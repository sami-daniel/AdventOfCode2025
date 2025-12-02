using System.Runtime.ExceptionServices;

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

                for (long i = first; i <= second; i++)
                {
                    total += ShouldSum(i) ? i : 0; 
                }
            }

            Console.WriteLine($"Output: {total}");
        }        

        static bool ShouldSum(long n, int endAt = 1)
        {
            var s = n.ToString();
            var pattern = s[..endAt];
            Console.WriteLine(s);
            Console.WriteLine(pattern);
            if (s.Length % pattern.Length == 0)
            {
                List<string> parts = [];
                var length = s.Length / pattern.Length;

                for (int i = 0; i < n; i++)
                {
                    Console.WriteLine(length);
                    parts.Add(s.Substring(i * length, length));
                }

                Console.WriteLine(parts);
            }

            return false;
        }
    }
}