using RanbowBack.DbObjects.Base;

namespace RanbowBack.DbObjects
{
    internal class DbOperator : DbObject
    {
#nullable enable
        public string? ImageUrl { get; set; }
        public int Primary1 { get; set; }
        public int? Primary2 { get; set; }
        public int? Primary3 { get; set; }
        public int Secondary1 { get; set; }
        public int? Secondary2 { get; set; }
        public int? Secondary3 { get; set; }
        public int Gadget1 { get; set; }
        public int Gadget2 { get; set; }
        public int? Gadget3 { get; set; }
        public string? Description { get; set; }
        public bool Attacker { get; set; }
#nullable disable
    }
}
