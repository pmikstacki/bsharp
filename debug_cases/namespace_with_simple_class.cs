namespace MyApp
{
    public class Calculator 
    {
        private int result;
        
        public void Add(int value) 
        {
            result = result + value;
        }
        
        public int GetResult() 
        {
            return result;
        }
    }
} 