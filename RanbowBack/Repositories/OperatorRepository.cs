using MySql.Data.MySqlClient;
using RanbowBack.Config;
using RanbowBack.DbObjects;
using RanbowBack.Models;
using RanbowBack.Repositories.Base;
using System;
using System.Collections.Generic;
using System.Diagnostics;

namespace RanbowBack.Repositories
{
	class OperatorRepository : BaseRepository
	{
		public OperatorRepository(string connectionString) : base(connectionString){ }

		public bool GetAll(out List<OperatorModel> list)
		{
			bool result = false;
			List<DbOperator> dbObjectList = new();

			try
			{
				DbCommand.CommandText = "SELECT * FROM Operators;";
				DbCommand.Parameters.Clear();
				DbConnection.Open();
				using (MySqlDataReader reader = DbCommand.ExecuteReader())
				{
					while (reader.Read())
					{
						dbObjectList.Add(new()
						{
							ID = (int)reader["ID"],
							Name = (string)reader["Name"],
							Primary1 = (int)reader["Primary1"],
							Primary2 = (int)reader["Primary2"],
							Primary3 = (int)reader["Primary3"],
							Secondary1 = (int)reader["Secondary1"],
							Secondary2 = (int)reader["Secondary2"],
							Secondary3 = (int)reader["Secondary3"],
							Gadget1 = (int)reader["Gadget1"],
							Gadget2 = (int)reader["Gadget2"],
							Description = (string)reader["Description"]
						});
					}
				}
				DbConnection.Close();

				result = true;
			}
			catch (Exception e)
			{
				//TODO: Add logging
				Debug.WriteLine(e.Message);
			}
			finally
			{
				if (DbConnection.State == System.Data.ConnectionState.Open)
				{
					DbConnection.Close();
				}
			}

			if (result && dbObjectList.Count != 0)
			{
				list = new();
				foreach (var item in dbObjectList)
				{
					OperatorModel model = new()
					{
						ID = item.ID,
						Name = item.Name,
						Description = item.Description
					};
					WeaponRepository weaponRepo = new(Configuration.Instance.Config.ConnectionString);

					#region accident
					//TODO: Clean this mess up
					if (!weaponRepo.Read(item.Primary1, out WeaponModel weapon))
					{
						//TODO: Add logging
						Debug.WriteLine("Weapon read failed for id: " + item.Primary1);
					}
					model.Primary1 = weapon;
					if (!weaponRepo.Read(item.Primary2, out weapon))
					{
						//TODO: Add logging
						Debug.WriteLine("Weapon read failed for id: " + item.Primary2);
					}
					model.Primary2 = weapon;
					if (!weaponRepo.Read(item.Primary3, out weapon))
					{
						//TODO: Add logging
						Debug.WriteLine("Weapon read failed for id: " + item.Primary3);
					}
					model.Primary3 = weapon;

					if (!weaponRepo.Read(item.Secondary1, out weapon))
					{
						//TODO: Add logging
						Debug.WriteLine("Weapon read failed for id: " + item.Secondary1);
					}
					model.Secondary1 = weapon;
					if (!weaponRepo.Read(item.Secondary2, out weapon))
					{
						//TODO: Add logging
						Debug.WriteLine("Weapon read failed for id: " + item.Secondary2);
					}
					model.Secondary2 = weapon;
					if (!weaponRepo.Read(item.Secondary3, out weapon))
					{
						//TODO: Add logging
						Debug.WriteLine("Weapon read failed for id: " + item.Secondary3);
					}
					model.Secondary3 = weapon;

					GadgetRepository gadgetRepo = new(Configuration.Instance.Config.ConnectionString);
					if (!gadgetRepo.Read(item.Gadget1, out GadgetModel gadget))
					{
						//TODO: Add logging
						Debug.WriteLine("Gadget read failed for id: " + item.Gadget1);
					}
					model.Gadget1 = gadget;
					if (!gadgetRepo.Read(item.Gadget2, out gadget))
					{
						//TODO: Add logging
						Debug.WriteLine("Weapon read failed for id: " + item.Gadget2);
					}
					model.Gadget2 = gadget;
					#endregion

					list.Add(model);
				}
				return result;
			}
			list = null;

			return false;
		}
	}
}
