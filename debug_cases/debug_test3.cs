namespace TestApp 
{
    public class ProductRepository : IRepository<Product>
    {
        public void Add(Product entity);
        public void Delete(int id);
        public Product GetById(int id);
    }
} 