using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace TestApp
{
    public interface IRepository<T>
    {
        T GetById(int id);
        IEnumerable<T> GetAll();
        void Add(T entity);
        void Update(T entity);
        void Delete(int id);
    }

    public class Product
    {
        public int Id { get; set; }
        public string Name { get; set; }
        public decimal Price { get; set; }
    }

    public class ProductRepository : IRepository<Product>
    {
        private readonly List<Product> _products = new List<Product>();

        public void Add(Product entity)
        {
            _products.Add(entity);
        }

        public void Delete(int id)
        {
            var product = GetById(id);
            if (product != null)
            {
                _products.Remove(product);
            }
        }

        public IEnumerable<Product> GetAll()
        {
            return _products;
        }

        public Product GetById(int id)
        {
            return _products.FirstOrDefault(p => p.Id == id);
        }

        public void Update(Product entity)
        {
            var product = GetById(entity.Id);
            if (product != null)
            {
                product.Name = entity.Name;
                product.Price = entity.Price;
            }
        }
    }

    public class AsyncExample
    {
        public async Task<string> FetchDataAsync()
        {
            await Task.Delay(1000); // Simulate API call
            return "Data fetched successfully";
        }
        
        public async Task ProcessDataAsync()
        {
            try
            {
                string data = await FetchDataAsync();
                Console.WriteLine(data);
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error: {ex.Message}");
            }
            finally
            {
                Console.WriteLine("Processing completed");
            }
        }
    }
}
