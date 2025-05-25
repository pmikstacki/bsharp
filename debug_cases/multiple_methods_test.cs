public class Test
{
    public void Add(Product entity)
    {
        _products.Add(entity);
    }

    public void Delete(int id)
    {
        var product = GetById(id);
    }
} 