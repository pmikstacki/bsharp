namespace TestApp 
{
    public class ProductRepository : IRepository<Product>
    {
        private readonly List<Product> _products = new List<Product>();

        public void Add(Product entity)
        {
            _products.Add(entity);
        }
    }
} 