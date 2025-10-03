using System;

namespace TestApp
{
    public class ControlFlowExamples
    {
        public void IfStatements(int value)
        {
            if (value > 10)
            {
                Console.WriteLine("Value is greater than 10");
            }
            else if (value > 5)
            {
                Console.WriteLine("Value is between 6 and 10");
            }
            else
            {
                Console.WriteLine("Value is 5 or less");
            }
        }

        public void SwitchStatement(string fruit)
        {
            switch (fruit.ToLower())
            {
                case "apple":
                    Console.WriteLine("Selected an apple");
                    break;
                case "banana":
                    Console.WriteLine("Selected a banana");
                    break;
                case "orange":
                    Console.WriteLine("Selected an orange");
                    break;
                default:
                    Console.WriteLine("Unknown fruit");
                    break;
            }
        }

        public void Loops()
        {
            for (int i = 0; i < 5; i++)
            {
                Console.WriteLine("For loop: " + i);
            }

            int counter = 0;
            while (counter < 3)
            {
                Console.WriteLine("While loop: " + counter);
                counter++;
            }

            int doCounter = 0;
            do
            {
                Console.WriteLine("Do-while loop: " + doCounter);
                doCounter++;
            } while (doCounter < 3);
        }
    }
}
