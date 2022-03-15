namespace RanbowBack.Models.Base
{
    public class BaseModel
    {
        public int ID { get; set; }
        public string Name { get; set; }

        public override string ToString()
        {
            return Name;
        }
    }
}
