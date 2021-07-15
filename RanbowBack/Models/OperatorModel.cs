using RanbowBack.Models.Base;

namespace RanbowBack.Models
{
	public class OperatorModel : BaseModel
	{
		public WeaponModel Primary1 { get; set; }
		public WeaponModel Primary2 { get; set; }
		public WeaponModel Primary3 { get; set; }
		public WeaponModel Secondary1 { get; set; }
		public WeaponModel Secondary2 { get; set; }
		public WeaponModel Secondary3 { get; set; }
		public GadgetModel Gadget1 { get; set; }
		public GadgetModel Gadget2 { get; set; }
		public string Description { get; set; }
	}
}
